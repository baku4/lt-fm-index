mod bwt;

use bwt::Bwt;

struct Config {
    // burrow wheeler transformed string (BWT)
    // bwt_segment_size: usize,
    // kmer lookup table
    lookup_kmer: Option<usize>,
    // occurrence array (OA)
    // suffix array (SA)
    sa_sampling_ratio: u64,
}

struct FmIndex {
    bwt: Bwt,
    count_array: CountArray,
    suffix_array: SuffixArray,
    sampling_ratio: u64,
}

impl FmIndex {
    #[inline]
    fn new(config: &Config, text: Vec<u8>) {
        // let bwt = Bwt::new();
        ()
    }
    #[inline]
    fn count(&self, pattern: &[u8]) {

    }
    #[inline]
    fn locate(&self, pattern: &[u8]) -> Vec<u64> {
        let mut idx = pattern.len();
        let c = pattern[idx-1];
        let mut pos_range = self.pos_range_init(c);
        // (1) LF mapping
        while pos_range.0 < pos_range.1 && idx > 0 {
            let c = pattern[idx-1];
            pos_range = self.bwt.lf_map_with_range(pos_range, c, &self.count_array);
            idx -= 1;
        }
        // (2) Locate 
        let pos_range_gap = pos_range.1 - pos_range.0;
        let mut locations: Vec<u64> = Vec::with_capacity(pos_range_gap as usize);
        for i in 0..pos_range_gap {
            let mut position = pos_range.0 + i;
            let mut offset: u64 = 0;
            while position % self.sampling_ratio != 0 {
                position = self.bwt.lf_map_with_pos(position, &self.count_array);
                offset += 1;
            }
            let location = self.suffix_array[(position / self.sampling_ratio) as usize] + offset;
            locations.push(location);
        }
        locations
    }
    #[inline]
    fn pos_range_init(&self, c: u8) -> (u64, u64) {
        let idx = nc_to_idx(&c);
        (self.count_array[idx], self.count_array[idx+1])
    }
}

// using 5 space for lessconditional statements
type CountArray = [u64; 5];

const A_UTF8: u8 = 65;
const C_UTF8: u8 = 67;
const G_UTF8: u8 = 71;
const T_UTF8: u8 = 84;

const A_U8_IDX: u8 = 0b00;
const C_U8_IDX: u8 = 0b01;
const G_U8_IDX: u8 = 0b10;
const T_U8_IDX: u8 = 0b11;

#[inline]
fn nc_to_idx(c: &u8) -> usize {
    match *c {
        A_UTF8 => 0,
        C_UTF8 => 1,
        G_UTF8 => 2,
        _ => 3,
    }
}

type KmerLookupTable = Vec<u64>;

struct BaseLookupTable {
    
}

type SuffixArray = Vec<u64>;

#[cfg(test)]
mod tests {
    use super::*;
    use libdivsufsort_rs::*;
    use radix_fmt::*;
    use std::fmt::Write;

    const A_UTF8: u8 = 65;
    const C_UTF8: u8 = 67;
    const G_UTF8: u8 = 71;
    const T_UTF8: u8 = 84;

    fn kmer_table_index(window: &[u8]) -> usize {
        window.iter().rev().enumerate().map(|(idx, c)| 
            4usize.pow(idx as u32) * match *c {
                A_UTF8 => 0,
                C_UTF8 => 1,
                G_UTF8 => 2,
                _ => 3, // do not check if there is only ACGT
            }
        ).sum()
    }

    fn accumulate_count_array(count_array: &mut [u64]) {
        let mut accumed_count: u64 = 0;
        count_array.iter_mut().for_each(|count| {
            accumed_count += *count;
            *count = accumed_count;
        });
    }

    fn accumulate_kmer_lookup_table(count_array: &mut [u64]) {
        let mut accumed_count: u64 = 0;
        count_array.iter_mut().for_each(|count| {
            accumed_count += *count;
            *count = accumed_count;
        });
    }

    fn print_kmer_lookup_table(table: &Vec<u64>, kmer_size: usize) {
        for (idx, count) in table.iter().enumerate() {
            if *count != 0 {
                let kmer_string = {
                    let mut index_radix = String::new();
                    let _ = write!(&mut index_radix, "{}", Radix::new(idx, 4));

                    let mut kmer_string = String::new();
                    for _ in 0..(kmer_size-index_radix.len()) {
                        kmer_string.push('A');
                    };

                    for c in index_radix.chars() {
                        let char = match c {
                            '0' => 'A',
                            '1' => 'C',
                            '2' => 'G',
                            '3' => 'T',
                            _ => panic!("lookup table only accept ACGT"),
                        };
                        kmer_string.push(char);
                    };
                    kmer_string
                };
                println!("{:?}: {:?}", kmer_string, count);
            }
        }
    }

    #[test]
    fn test() {
        let input_string = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAG".as_bytes().to_vec();
        // sa
        let suffix_array = divsufsort64(&input_string).unwrap();
        // bwt
        let (bwt, pidx) = {
            let mut bwt = input_string.clone();
            let mut sa = suffix_array.clone();
            let pidx = bw_transform64(&mut bwt, &mut sa).unwrap();
            (bwt, pidx)
        };
        println!("input_string:\n{:?}", String::from_utf8(input_string.clone()).unwrap());
        println!("sa:\n{:?}", suffix_array);
        println!("bwt:\n{:?}", String::from_utf8(bwt.clone()).unwrap());
        println!("pidx:\n{:?}", pidx);
        // count array & kmer lookup table
        let (count_array, kmer_lookup_table) = {
            let mut count_array = [0; 4];
            let kmer: usize = 8;
            let mut kmer_lookup_table: Vec<u64> = vec![0; 4usize.pow(kmer as u32)];
            let mut kmer_iter = input_string[..].windows(kmer);
            while let Some(v) = kmer_iter.next() {
                let table_index = kmer_table_index(v);
                kmer_lookup_table[table_index] += 1;
                match v[0] {
                    A_UTF8 => count_array[0] += 1,
                    C_UTF8 => count_array[1] += 1,
                    G_UTF8 => count_array[2] += 1,
                    _ => count_array[3] += 1,
                }
            };
            // TODO: add count of string containing primary index($) to KLT 
            accumulate_count_array(&mut count_array);
            accumulate_kmer_lookup_table(&mut kmer_lookup_table);
            (count_array, kmer_lookup_table)
        };
        println!("ca:\n{:?}", count_array);
    }
}
