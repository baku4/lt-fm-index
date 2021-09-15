use super::{Text, Serialize, Deserialize};

use libdivsufsort_rs::{divsufsort64, bw_transform64};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct SuffixArray {
    pub sampling_ratio: u64,
    pub array: Vec<u64>,
}

impl SuffixArray {
    pub fn new_while_bwt(text: &mut Text, sa_sampling_ratio: u64) -> (Self, u64) {
        let suffix_array_i64 = divsufsort64(&text).unwrap();
        let pidx = {
            let mut sa = suffix_array_i64.clone();
            let pidx = bw_transform64(text, &mut sa).unwrap();
            pidx
        };
        let compressed_array = Self::compress_suffix_array(suffix_array_i64, sa_sampling_ratio);
        let suffix_array = Self {
            sampling_ratio: sa_sampling_ratio,
            array: compressed_array,
        };
        (suffix_array, pidx as u64)
    }
    fn compress_suffix_array(suffix_array: Vec<i64>, sampling_ratio: u64) -> Vec<u64> {
        if sampling_ratio == 1 {
            suffix_array.into_iter().map(|x| x as u64).collect()
        } else {
            suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x as u64).collect()
        }
    }
}