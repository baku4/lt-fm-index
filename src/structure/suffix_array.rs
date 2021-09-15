use super::Text;

use libdivsufsort_rs::{divsufsort64, bw_transform64};

pub fn get_suffix_array_and_pidx_with_bw_transforming(text: &mut Text, sa_sampling_ratio: u64) -> (Vec<u64>, u64) {
    let suffix_array_i64 = divsufsort64(&text).unwrap();
    let pidx = {
        let mut sa = suffix_array_i64.clone();
        let pidx = bw_transform64(text, &mut sa).unwrap();
        pidx
    };
    let suffix_array = compress_suffix_array(suffix_array_i64, sa_sampling_ratio);
    (suffix_array, pidx as u64)
}

fn compress_suffix_array(suffix_array: Vec<i64>, sampling_ratio: u64) -> Vec<u64> {
    if sampling_ratio == 1 {
        suffix_array.into_iter().map(|x| x as u64).collect()
    } else {
        suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x as u64).collect()
    }
}