mod bwt;

use bwt::Bwt;
use libdivsufsort_rs::{divsufsort64, bw_transform64};

struct Config {
    // burrow wheeler transformed string (BWT)
    // bwt_segment_size: usize,
    // kmer lookup table
    kmer_size: Option<usize>,
    // suffix array (SA)
    sa_sampling_ratio: u64,
}

struct FmIndex {
    count_array: CountArray,
    sampling_ratio: u64,
    text_len: u64,
    suffix_array: SuffixArray,
    kmer_lookup_table: Option<KmerLookupTable>,
    bwt: Bwt,
}

impl FmIndex {
    #[inline]
    pub fn new(config: &Config, text: Vec<u8>) -> Self {
        let text_len = text.len() as u64;
        // suffix_array
        let suffix_array = divsufsort64(&text).unwrap();
        // bwt & primary index
        let (bwt_string, pidx) = {
            let mut bwt = text.clone();
            let mut sa = suffix_array.clone();
            let pidx = bw_transform64(&mut bwt, &mut sa).unwrap();
            (bwt, pidx)
        };
        // compress suffix array
        let suffix_array = compress_suffix_array(suffix_array, config.sa_sampling_ratio);
        let bwt = Bwt::new(bwt_string, pidx);
        // count array
        let (count_array, kmer_lookup_table): (CountArray, Option<KmerLookupTable>) = match config.kmer_size {
            Some(kmer) => {
                let mut count_array: CountArray = [0; 5];
                let mut kmer_lookup_table: Vec<u64> = vec![0; 4usize.pow(kmer as u32)];
                let mut kmer_iter = text[..].windows(kmer);
                while let Some(v) = kmer_iter.next() {
                    // Accum kmer table
                    let table_index = kmer_table_index(v);
                    kmer_lookup_table[table_index] += 1;
                    // Accum count array
                    match v[0] {
                        A_UTF8 => count_array[1] += 1,
                        C_UTF8 => count_array[2] += 1,
                        G_UTF8 => count_array[3] += 1,
                        _ => count_array[4] += 1,
                    }
                };
                // dealing with last k-1 string
                let mut table_index: usize = 0;
                let pow = 4_usize.pow(kmer as u32 - 1);
                for c in text[text_len as usize-kmer+1..].iter().rev() {
                    match *c {
                        A_UTF8 => {
                            table_index /= 4;
                            count_array[1] += 1;
                        },
                        C_UTF8 => {
                            table_index /= 4;
                            table_index += pow;
                            count_array[2] += 1;
                        },
                        G_UTF8 => {
                            table_index /= 4;
                            table_index += 2*pow;
                            count_array[3] += 1;
                        },
                        _ => {
                            table_index /= 4;
                            table_index += 3*pow;
                            count_array[4] += 1;
                        },
                    }
                    kmer_lookup_table[table_index] += 1;
                }
                accumulate_count_array(&mut count_array);
                accumulate_count_array(&mut kmer_lookup_table);
                (count_array, Some((kmer, kmer_lookup_table)))
            },
            None => {
                let mut count_array: CountArray = [0; 5];
                for c in &text {
                    match *c {
                        A_UTF8 => count_array[1] += 1,
                        C_UTF8 => count_array[2] += 1,
                        G_UTF8 => count_array[3] += 1,
                        _ => count_array[4] += 1,
                    }
                }
                accumulate_count_array(&mut count_array);
                (count_array, None)
            }
        };
        Self {
            count_array: count_array,
            sampling_ratio: config.sa_sampling_ratio,
            suffix_array: suffix_array,
            text_len: text_len,
            kmer_lookup_table: kmer_lookup_table,
            bwt: bwt,
        }
    }
    #[inline]
    pub fn count(&self, pattern: &[u8]) -> u64 {
        let pos_range = self.lf_map(pattern);
        pos_range.1 - pos_range.0
    }
    #[inline]
    pub fn locate(&self, pattern: &[u8]) -> Vec<u64> {
        let pos_range = self.lf_map(pattern);
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        for mut position in pos_range.0..pos_range.1 {
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
    fn lf_map(&self, pattern: &[u8]) -> (u64, u64) {
        let mut idx = pattern.len();
        let c = pattern[idx-1];
        let mut pos_range = self.pos_range_init(c);
        idx -= 1;
        // LF mapping
        while pos_range.0 < pos_range.1 && idx > 0 {
            let c = pattern[idx-1];
            pos_range = self.bwt.lf_map_with_range(pos_range, c, &self.count_array);
            idx -= 1;
        }
        pos_range
    }
    #[inline]
    fn pos_range_init(&self, c: u8) -> (u64, u64) {
        let idx = nc_to_idx(&c);
        (self.count_array[idx], self.count_array[idx+1])
    }
    #[inline]
    pub fn locate_with_klt(&self, pattern: &[u8]) -> Vec<u64> {
        let (kmer_size, klt) = self.kmer_lookup_table.as_ref().unwrap();
        let mut idx = pattern.len();
        let pattern_len = idx.clone() as u64;
        let mut pos_range: (u64, u64) = {
            if *kmer_size >= idx {
                let (start_klt_idx, end_kit_idx) = kmer_table_index_from_smaller_string(pattern, kmer_size);
                idx -= idx;
                if start_klt_idx == 0 {
                    (0, klt[end_kit_idx])
                } else {
                    (klt[start_klt_idx-1], klt[end_kit_idx])
                }
            } else {
                let kmer_window = &pattern[idx-kmer_size..];
                let klt_idx = kmer_table_index(kmer_window);
                idx -= kmer_size;
                if klt_idx == 0 {
                    (0, klt[klt_idx])
                } else {
                    (klt[klt_idx-1], klt[klt_idx])
                }
            }
        };
        while pos_range.0 < pos_range.1 && idx > 0 {
            let c = pattern[idx-1];
            pos_range = self.bwt.lf_map_with_range(pos_range, c, &self.count_array);
            idx -= 1;
        }
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        for mut position in pos_range.0..pos_range.1 {
            let mut offset: u64 = 0;
            while position % self.sampling_ratio != 0 {
                position = self.bwt.lf_map_with_pos(position, &self.count_array);
                offset += 1;
            }
            let location = self.suffix_array[(position / self.sampling_ratio) as usize] + offset;
            // check if valid location
            if location + pattern_len <= self.text_len {
                locations.push(location);
            }
        }
        locations
    }
}

// using 5 space for lessconditional statements
type CountArray = [u64; 5];

#[inline]
fn accumulate_count_array(count_array: &mut [u64]) {
    let mut accumed_count: u64 = 0;
    count_array.iter_mut().for_each(|count| {
        accumed_count += *count;
        *count = accumed_count;
    });
}

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

type KmerLookupTable = (usize, Vec<u64>); // kmer_size, table vector

#[inline]
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
#[inline]
fn kmer_table_index_from_smaller_string(window: &[u8], kmer: &usize) -> (usize, usize) {
    let mut table_index: usize = 0;
    let pow = 4_usize.pow(*kmer as u32 - 1);
    window.iter().rev().for_each(|c| {
        match *c {
            A_UTF8 => {
                table_index /= 4;
            },
            C_UTF8 => {
                table_index /= 4;
                table_index += pow;
            },
            G_UTF8 => {
                table_index /= 4;
                table_index += 2*pow;
            },
            _ => {
                table_index /= 4;
                table_index += 3*pow;
            },
        }
    });
    let offset = 4_usize.pow((*kmer - window.len()) as u32);
    (table_index, table_index + offset - 1)
}

type SuffixArray = Vec<u64>;

#[inline]
fn compress_suffix_array(suffix_array: Vec<i64>, sampling_ratio: u64) -> SuffixArray {
    suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x as u64).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use libdivsufsort_rs::*;

    // For cross check
    use fm_index::converter::RangeConverter;
    use fm_index::suffix_array::SuffixOrderSampler;
    use fm_index::{BackwardSearchIndex, FMIndex};

    fn get_locations_using_other_crate(text: &Vec<u8>, pattern: &Vec<u8>) -> Vec<u64> {
        let converter = RangeConverter::new(b' ', b'~');
        let sampler = SuffixOrderSampler::new().level(2);
        let index = FMIndex::new(text.clone(), converter, sampler);
        let search = index.search_backward(pattern);
        search.locate()
    }

    #[test]
    fn test_compress_suffix_array() {
        let raw_suffix_array: Vec<i64> = (0..30).collect();
        let sampling_ratio: u64 = 5;
        let sa = compress_suffix_array(raw_suffix_array, sampling_ratio);
        assert_eq!(sa, vec![0, 5, 10, 15, 20, 25]);
    }

    #[test]
    fn test_fm_index_locate() {
        let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let config = Config {
            sa_sampling_ratio: 4,
            kmer_size: None,
        };
        let fm_index = FmIndex::new(&config, text.clone());
        // test
        for pattern in vec!["TA", "T", "AAGTGAAATTTCCACATCGCCGGAAAC", "AA", "GGC"] {
            let pattern = pattern.as_bytes().to_vec();
            let mut locations_res = fm_index.locate(&pattern);
            locations_res.sort();
            let mut locations_ans = get_locations_using_other_crate(&text, &pattern.to_vec());
            locations_ans.sort();
            assert_eq!(locations_res, locations_ans);
        }
    }

    #[test]
    fn test_fm_index_locate_with_klt() {
        let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let config = Config {
            sa_sampling_ratio: 4,
            kmer_size: Some(7),
        };
        let fm_index = FmIndex::new(&config, text.clone());
        // test
        for pattern in vec!["TA", "T", "AAGTGAAATTTCCACATCGCCGGAAAC", "AA", "GGC"] {
            let pattern = pattern.as_bytes().to_vec();
            let mut locations_res = fm_index.locate_with_klt(&pattern);
            locations_res.sort();
            let mut locations_ans = get_locations_using_other_crate(&text, &pattern.to_vec());
            locations_ans.sort();
            assert_eq!(locations_res, locations_ans);
        }
    }
}
