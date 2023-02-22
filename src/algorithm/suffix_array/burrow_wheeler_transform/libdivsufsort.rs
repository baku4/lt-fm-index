use crate::{TextLength, Text};

#[cfg(not(features = "longtext"))]
use libdivsufsort_rs::{
    divsufsort as divsufsort,
    bw_transform as bw_transform,
};
#[cfg(features = "longtext")]
use libdivsufsort_rs::{
    divsufsort64 as divsufsort,
    bw_transform64 as bw_transform,
};

#[inline]
pub fn get_compressed_suffix_array_and_pidx_while_bwtt_with_libdivsufsort(text: &mut Text, sampling_ratio: TextLength) -> (Vec<TextLength>, TextLength) {
    let suffix_array = divsufsort(text).unwrap();
    let pidx = {
        let mut sa = suffix_array.clone();
        let pidx = bw_transform(text, &mut sa).unwrap();
        pidx
    };

    let compressed_suffix_array = suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x as TextLength).collect();
    (compressed_suffix_array, pidx as TextLength)
}
