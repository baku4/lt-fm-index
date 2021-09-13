mod bwt_on;

use super::{FmIndexConfigDep, LtFmIndexDep};
use super::utils::{
    SuffixArray,
    accumulate_count_array, compress_suffix_array,
};
use bwt_on::BwtOn;
use libdivsufsort_rs::{divsufsort64, bw_transform64};
use serde::{Serialize, Deserialize};

/* Types */
type CountArray = [u64; 5]; // using 5 space for lessconditional statements
type KmerLookupTable = (usize, Vec<u64>); // kmer_size, table vector

/* Char Encoding */
const A_UTF8: u8 = 65;
const C_UTF8: u8 = 67;
const G_UTF8: u8 = 71;
const T_UTF8: u8 = 84;

const A_U8_IDX: u8 = 0b00;
const C_U8_IDX: u8 = 0b01;
const G_U8_IDX: u8 = 0b10;
const T_U8_IDX: u8 = 0b11;

const A_IDX: usize = 0b000;
const C_IDX: usize = 0b001;
const G_IDX: usize = 0b010;
const T_IDX: usize = 0b011;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Lt-Fm-index data structure
pub struct FmIndexOn {
    pub count_array: CountArray,
    pub sampling_ratio: u64,
    pub text_len: u64,
    pub suffix_array: SuffixArray,
    pub kmer_lookup_table: Option<KmerLookupTable>,
    pub bwt: BwtOn,
}


impl FmIndexOn {
    /// Create new fm-index with configuration
    #[inline]
    pub fn new(config: &FmIndexConfigDep, mut text: Vec<u8>) -> Self {
        let text_len = text.len() as u64;
        // (1) count array & klt
        let (count_array, kmer_lookup_table): (CountArray, Option<KmerLookupTable>) = Self::get_ca_and_klt(&config, &mut text);
        // (2) suffix_array
        let suffix_array = divsufsort64(&text).unwrap();
        // (3) bwt & primary index
        let pidx = {
            let mut sa = suffix_array.clone();
            let pidx = bw_transform64(&mut text, &mut sa).unwrap();
            pidx
        }; // original text is trasformed to bwt string
        // (4) compression
        let suffix_array = compress_suffix_array(suffix_array, config.sa_sampling_ratio);
        let bwt = BwtOn::new(text, pidx);
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
    pub fn get_ca_and_klt(config: &FmIndexConfigDep, text: &mut Vec<u8>) -> (CountArray, Option<KmerLookupTable>) {
        match config.kmer_size {
            Some(kmer) => {
                // Init
                let mut count_array: CountArray = [0; 5];
                let klt_length: usize = 4usize.pow(kmer as u32);
                let mut kmer_lookup_table: Vec<u64> = vec![0; klt_length];
                let mut klt_index: usize = 0;
                let index_truncating_bits: usize = klt_length-1;
                // Use each char
                //  - first k-1
                text[..kmer-1].iter().for_each(|word| {
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
                        _ => {
                            count_array[4] += 1;
                            klt_index <<= 2;
                            klt_index += 3;
                        },
                    }
                });
                //  - kmer to end
                text[kmer-1..].iter().for_each(|word| {
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
                        _ => {
                            count_array[4] += 1;
                            klt_index <<= 2;
                            klt_index += 3;
                            klt_index &= index_truncating_bits;
                            kmer_lookup_table[klt_index] += 1;
                        },
                    }
                });
                //  - last k
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
                let mut count_array: CountArray = [0; 5];
                for c in text {
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
        }
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

impl LtFmIndexDep for FmIndexOn {
    /// Count the number of pattern in the text
    #[inline]
    fn count(&self, pattern: &[u8]) -> u64 {
        let pos_range = self.lf_map(pattern);
        pos_range.1 - pos_range.0
    }
    /// Locate index of the pattern in the text (not use k-mer lookup table)
    #[inline]
    fn locate_wo_klt(&self, pattern: &[u8]) -> Vec<u64> {
        let pos_range = self.lf_map(pattern);
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        'each_pos: for mut position in pos_range.0..pos_range.1 {
            let mut offset: u64 = 0;
            while position % self.sampling_ratio != 0 {
                match self.bwt.get_pre_pos(position, &self.count_array) {
                    Some(v) => {
                        position = v;
                    },
                    None => { // if position == pidx
                        locations.push(offset);
                        continue 'each_pos;
                    },
                }
                offset += 1;
            }
            let location = self.suffix_array[(position / self.sampling_ratio) as usize] + offset;
            locations.push(location);
        }
        locations
    }
    /// Locate index of the pattern in the text with k-mer lookup table
    #[inline]
    fn locate_w_klt(&self, pattern: &[u8]) -> Vec<u64> {
        let (kmer_size, klt) = self.kmer_lookup_table.as_ref().unwrap();
        let mut idx = pattern.len();
        let pattern_len = idx.clone() as u64;
        // init pos range using KLT
        let mut pos_range: (u64, u64) = {
            // get index of klt
            if *kmer_size < idx {
                let klt_index = klt_index_of_pattern(&pattern[idx-*kmer_size..]);
                idx -= *kmer_size;
                if klt_index == 0 {
                    (0, klt[klt_index])
                } else {
                    (klt[klt_index-1], klt[klt_index])
                }
            } else {
                let klt_index = klt_index_of_pattern(pattern);
                let offset_adder = 4_usize.pow((*kmer_size - idx) as u32);
                let klt_index_start = klt_index * offset_adder;
                let klt_index_end = klt_index * offset_adder + offset_adder - 1;
                idx = 0;
                if klt_index_start == 0 {
                    (0, klt[klt_index_end])
                } else {
                    (klt[klt_index_start-1], klt[klt_index_end])
                }
            }
        };
        while pos_range.0 < pos_range.1 && idx > 0 {
            let c = pattern[idx-1];
            pos_range = self.bwt.next_pos_range_from_range(pos_range, c, &self.count_array);
            idx -= 1;
        }
        let mut locations: Vec<u64> = Vec::with_capacity((pos_range.1 - pos_range.0) as usize);
        'each_pos: for mut position in pos_range.0..pos_range.1 {
            let mut offset: u64 = 0;
            while position % self.sampling_ratio != 0 {
                match self.bwt.get_pre_pos(position, &self.count_array) {
                    Some(v) => {
                        position = v;
                        offset += 1;
                    },
                    None => { // if position == pidx
                        locations.push(offset);
                        continue 'each_pos;
                    },
                }
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

/* CHRACTER ENCODING */
#[inline]
fn nc_to_idx(c: &u8) -> usize {
    match *c {
        A_UTF8 => A_IDX,
        C_UTF8 => C_IDX,
        G_UTF8 => G_IDX,
        _ => T_IDX,
    }
}

/* KMER INDEX TABLE */
#[inline]
fn klt_index_of_pattern(pattern: &[u8]) -> usize {
    let mut klt_index: usize = 0;
    pattern.iter().for_each(|chr| {
        match *chr {
            A_UTF8 => {
                klt_index <<= 2;
            },
            C_UTF8 => {
                klt_index <<= 2;
                klt_index += 1;
            },
            G_UTF8 => {
                klt_index <<= 2;
                klt_index += 2;
            },
            _ => {
                klt_index <<= 2;
                klt_index += 3;
            },
        }
    });
    klt_index
}
