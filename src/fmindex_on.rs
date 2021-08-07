mod bwt_on;

use super::{Config, FmIndexTrait};
use bwt_on::Bwt;
use libdivsufsort_rs::{divsufsort64, bw_transform64};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
/// Lt-Fm-index data structure
pub struct FmIndexOn {
    count_array: CountArray,
    sampling_ratio: u64,
    text_len: u64,
    suffix_array: SuffixArray,
    kmer_lookup_table: Option<KmerLookupTable>,
    bwt: Bwt,
}

impl FmIndexTrait for FmIndexOn {
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
                position = self.bwt.lf_map_with_pos(position, &self.count_array);
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

impl FmIndexOn {
    /// Create new fm-index with configuration
    #[inline]
    pub fn new(config: &Config, mut text: Vec<u8>) -> Self {
        let text_len = text.len() as u64;
        // count array
        let (count_array, kmer_lookup_table): (CountArray, Option<KmerLookupTable>) = Self::get_ca_and_klt(&config, &mut text);
        // suffix_array
        let suffix_array = divsufsort64(&text).unwrap();
        // bwt & primary index
        // original text is trasformed to bwt string
        let pidx = {
            let mut sa = suffix_array.clone();
            let pidx = bw_transform64(&mut text, &mut sa).unwrap();
            pidx
        };
        // compress suffix array
        let suffix_array = compress_suffix_array(suffix_array, config.sa_sampling_ratio);
        let bwt = Bwt::new(text, pidx);
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
                for c in text[text.len() as usize-kmer+1..].iter().rev() {
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
    if sampling_ratio == 1 {
        suffix_array.into_iter().map(|x| x as u64).collect()
    } else {
        suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x as u64).collect()
    }
}
