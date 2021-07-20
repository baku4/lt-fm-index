struct Meta {
    // burrow wheeler transformed string (BWT)
    bwt_compression_size: BwtCompressionSize,
    // kmer lookup table
    lookup_kmer: usize,
    // occurrence array (OA)
    // base lookup table
    lookup_base: usize,
    // suffix array (SA)
    sa_sampling_ratio: usize,
}

struct FmIndex {
    bwt: Bwt,
    oa: OccurrenceArray,
    ca: CountArray,
    sa: SuffixArray,
}

type CountArray = [u64; 4];
struct KmerLookupTable {

}

enum BwtCompressionSize {
    _32,
    _64,
    _128,
    _256,
}

struct Bwt {
    //
}

struct SuffixArray {
    //
}

struct BaseLookupTable {
    
}

struct OccurrenceArray {
    //
}

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
        // count array & kmer lookup_table
        let (count_array, kmer_lookup_table) = {
            let mut count_array: CountArray = [0; 4];
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
