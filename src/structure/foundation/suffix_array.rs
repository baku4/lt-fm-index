use super::{Text, Archive, Serialize, Deserialize};

#[allow(dead_code)]
mod bwt_transform;

#[cfg(not(target_arch = "wasm32"))]
use libdivsufsort_rs::{divsufsort64, bw_transform64};

#[derive(Debug, Archive, Serialize, Deserialize, Clone)]
#[archive(archived = "SuffixArray")]
pub struct SuffixArrayPreBuild {
    pub sampling_ratio: u64,
    pub array: Vec<u64>,
}

impl SuffixArrayPreBuild {
    #[cfg(not(target_arch = "wasm32"))]
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
    #[cfg(target_arch = "wasm32")]
    pub fn new_while_bwt(text: &mut Text, sa_sampling_ratio: u64) -> (Self, u64) {
        let (suffix_array_i64, bwt, pidx) = bwt_transform::burrow_wheeler_transform(&text);
        *text = bwt;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::random_text::*;

    #[test]
    #[cfg(not(target_arch = "wasm32"))]
    fn test_built_in_bwt_transform() {
        let n_test = 1000;

        for _ in 0..n_test {
            let text_no = rand_text_of_no();
            assert_built_in_bwt_same_with_libdivsufsort_rs(&text_no);
            let text_nn = rand_text_of_nn();
            assert_built_in_bwt_same_with_libdivsufsort_rs(&text_nn);
            let text_ao = rand_text_of_ao();
            assert_built_in_bwt_same_with_libdivsufsort_rs(&text_ao);
            let text_an = rand_text_of_an();
            assert_built_in_bwt_same_with_libdivsufsort_rs(&text_an);
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn assert_built_in_bwt_same_with_libdivsufsort_rs(text: &[u8]) {
        // Result from libdivsufsort_rs
        let mut bwt_answer = text.to_vec();
        let suffix_array_answer = divsufsort64(&bwt_answer).unwrap();
        let pidx_answer = {
            let mut sa = suffix_array_answer.clone();
            let pidx = bw_transform64(&mut bwt_answer, &mut sa).unwrap();
            pidx as u64
        };

        // Result from built_in_bwt
        let (suffix_array, bwt, pidx) = bwt_transform::burrow_wheeler_transform(&text);
        
        assert_eq!(suffix_array, suffix_array_answer);
        assert_eq!(bwt, bwt_answer);
        assert_eq!(pidx, pidx_answer);
    }
}