mod bwt_nn;

use super::{Config, FmIndexTrait};
use bwt_nn::BwtNn;
use libdivsufsort_rs::{divsufsort64, bw_transform64};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Lt-Fm-index data structure
pub struct FmIndexNn {
    count_array: CountArray,
    sampling_ratio: u64,
    text_len: u64,
    suffix_array: SuffixArray,
    kmer_lookup_table: Option<KmerLookupTable>,
    bwt: BwtNn,
}

impl FmIndexTrait for FmIndexNn {
    /// Count the number of pattern in the text
    #[inline]
    fn count(&self, pattern: &[u8]) -> u64 {
        let pos_range = self.lf_map(pattern);
        pos_range.1 - pos_range.0
    }
    /// Locate index of the pattern in the text (not use k-mer lookup table)
    #[inline]
    fn locate(&self, pattern: &[u8]) -> Vec<u64> {
        let pos_range = self.lf_map(pattern);
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        for mut position in pos_range.0..pos_range.1 {
            let mut offset: u64 = 0;
            while position % self.sampling_ratio != 0 {
                position = self.bwt.get_pre_pos(position, &self.count_array);
                offset += 1;
            }
            let location = self.suffix_array[(position / self.sampling_ratio) as usize] + offset;
            locations.push(location);
        }
        locations
    }
    /// Locate index of the pattern in the text with k-mer lookup table
    #[inline]
    fn locate_with_klt(&self, pattern: &[u8]) -> Vec<u64> {
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
                let klt_idx = kmer_table_index_dep(kmer_window);
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
            pos_range = self.bwt.next_pos_range_from_range(pos_range, c, &self.count_array);
            idx -= 1;
        }
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        for mut position in pos_range.0..pos_range.1 {
            let mut offset: u64 = 0;
            while position % self.sampling_ratio != 0 {
                position = self.bwt.get_pre_pos(position, &self.count_array);
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

impl FmIndexNn {
    /// Create new fm-index with configuration
    #[inline]
    pub fn new(config: &Config, mut text: Vec<u8>) -> Self {
        let text_len = text.len() as u64;
        // (1) count array & klt
        let (count_array, kmer_lookup_table): (CountArray, Option<KmerLookupTable>) = Self::get_ca_and_klt(config, &mut text);
        // (2) suffix_array
        let suffix_array = divsufsort64(&text).unwrap();
        // (3) bwt & primary index
        // original text is trasformed to bwt string
        let pidx = {
            let mut sa = suffix_array.clone();
            let pidx = bw_transform64(&mut text, &mut sa).unwrap();
            pidx
        };
        // (4) compression
        let suffix_array = compress_suffix_array(suffix_array, config.sa_sampling_ratio);
        let bwt = BwtNn::new(text, pidx);
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
    pub fn get_ca_and_klt(config: &Config, text: &mut Vec<u8>) -> (CountArray, Option<KmerLookupTable>) {
        match config.kmer_size {
            Some(kmer) => {
                // INIT
                let mut count_array: CountArray = [0; 6];
                let klt_length: usize = 4usize.pow(kmer as u32);
                let mut kmer_lookup_table: Vec<u64> = vec![0; klt_length];
                let mut klt_index: usize = 0;
                let index_truncating_bits: usize = klt_length-1;
                // ITERATING
                // (1) first k-1
                text[..kmer-1].iter_mut().for_each(|word| {
                    match *word {
                        A_UTF8 => {
                            count_array[1] += 1;
                            klt_index <<= 2;
                        },
                        C_UTF8 => {
                            count_array[2] += 1;
                            klt_index <<= 2;
                            klt_index += 1;
                        },
                        G_UTF8 => {
                            count_array[3] += 1;
                            klt_index <<= 2;
                            klt_index += 2;
                        },
                        T_UTF8 => {
                            count_array[4] += 1;
                            klt_index <<= 2;
                            klt_index += 3;
                        },
                        _ => {
                            count_array[5] += 1;
                            klt_index = 0;
                            *word = X_UTF8;
                        },
                    }
                });
                // (2) kmer to end
                text[kmer-1..].iter_mut().for_each(|word| {
                    match *word {
                        A_UTF8 => {
                            count_array[1] += 1;
                            klt_index <<= 2;
                            klt_index &= index_truncating_bits;
                            kmer_lookup_table[klt_index] += 1;
                        },
                        C_UTF8 => {
                            count_array[2] += 1;
                            klt_index <<= 2;
                            klt_index += 1;
                            klt_index &= index_truncating_bits;
                            kmer_lookup_table[klt_index] += 1;
                        },
                        G_UTF8 => {
                            count_array[3] += 1;
                            klt_index <<= 2;
                            klt_index += 2;
                            klt_index &= index_truncating_bits;
                            kmer_lookup_table[klt_index] += 1;
                        },
                        T_UTF8 => {
                            count_array[4] += 1;
                            klt_index <<= 2;
                            klt_index += 3;
                            klt_index &= index_truncating_bits;
                            kmer_lookup_table[klt_index] += 1;
                        },
                        _ => {
                            count_array[5] += 1;
                            klt_index = 0;
                            *word = X_UTF8;
                        },
                    }
                });
                // (3) last k
                (0..kmer-1).for_each(|_| {
                    klt_index <<= 2;
                    klt_index &= index_truncating_bits;
                    kmer_lookup_table[klt_index] += 1;
                });
                // accumulate array
                accumulate_count_array(&mut count_array);
                accumulate_count_array(&mut kmer_lookup_table);
                (count_array, Some((kmer, kmer_lookup_table)))
            },
            None => {
                let mut count_array: CountArray = [0; 6];
                for c in text {
                    match *c {
                        A_UTF8 => count_array[1] += 1,
                        C_UTF8 => count_array[2] += 1,
                        G_UTF8 => count_array[3] += 1,
                        T_UTF8 => count_array[4] += 1,
                        _ => count_array[5] += 1,
                    }
                }
                accumulate_count_array(&mut count_array);
                (count_array, None)
            }
        }
    }
    #[inline]
    fn lf_map(&self, pattern: &[u8]) -> (u64, u64) {
        let mut idx = pattern.len();
        let chr = pattern[idx-1];
        let mut pos_range = self.pos_range_init(chr);
        idx -= 1;
        // LF mapping
        while pos_range.0 < pos_range.1 && idx > 0 {
            let c = pattern[idx-1];
            pos_range = self.bwt.next_pos_range_from_range(pos_range, c, &self.count_array);
            idx -= 1;
        }
        pos_range
    }
    #[inline]
    fn pos_range_init(&self, c: u8) -> (u64, u64) {
        let idx = nc_to_idx(&c);
        (self.count_array[idx], self.count_array[idx+1])
    }
}

// using 6 space for less conditional statements
type CountArray = [u64; 6];

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
const X_UTF8: u8 = 88;

const A_U8_IDX: u8 = 0b000;
const C_U8_IDX: u8 = 0b001;
const G_U8_IDX: u8 = 0b010;
const T_U8_IDX: u8 = 0b011;
const X_U8_IDX: u8 = 0b100;

const A_IDX: usize = 0b000;
const C_IDX: usize = 0b001;
const G_IDX: usize = 0b010;
const T_IDX: usize = 0b011;
const X_IDX: usize = 0b100;

#[inline]
fn nc_to_idx(c: &u8) -> usize {
    match *c {
        A_UTF8 => A_IDX,
        C_UTF8 => C_IDX,
        G_UTF8 => G_IDX,
        T_UTF8 => T_IDX,
        _ => X_IDX,
    }
}

type KmerLookupTable = (usize, Vec<u64>); // kmer_size, table vector

#[inline]
fn kmer_table_index(pattern: &[u8], kmer: &usize) -> usize {
    let mut klt_index: usize = 0;
    let mut offset = kmer.clone();
    for word in pattern.iter() {
        match *word {
            A_UTF8 => {
                klt_index <<= 2;
                offset -= 1;
            },
            C_UTF8 => {
                klt_index <<= 2;
                klt_index += 1;
                offset -= 1;
            },
            G_UTF8 => {
                klt_index <<= 2;
                klt_index += 2;
                offset -= 1;
            },
            T_UTF8 => {
                klt_index <<= 2;
                klt_index += 3;
                offset -= 1;
            },
            _ => {
                break;
            },
        }
    };
    klt_index <<= 2*offset;
    klt_index
}
#[inline]
fn kmer_table_index_dep(window: &[u8]) -> usize {
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
    if sampling_ratio == 1 {
        suffix_array.into_iter().map(|x| x as u64).collect()
    } else {
        suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x as u64).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;

    #[test]
    fn test_get_ca_and_klt() {
        let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let config = Config::new()
            .set_suffix_array_sampling_ratio(4)
            .set_kmer_lookup_table(8);
        let (ca_1, klt_1) = {
            let mut cloned_text = text.clone();
            FmIndexNn::get_ca_and_klt(&config, &mut cloned_text)
        };
        let (ca_2, klt_2) = {
            let mut cloned_text = text.clone();
            fmindex_on::FmIndexOn::get_ca_and_klt(&config, &mut cloned_text)
        };
        println!("{:?}\n{:?}", ca_1, ca_2);
        assert_eq!(klt_1, klt_2);
    }
}