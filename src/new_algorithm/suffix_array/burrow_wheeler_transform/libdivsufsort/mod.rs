use crate::core::Position;
// Always use 64 bit version because the 32 bit version return the i32, smaller than u32.
use libdivsufsort_rs::{
    divsufsort64 as divsufsort,
    bw_transform64 as bw_transform,
};

#[inline]
pub fn get_compressed_suffix_array_and_pidx_while_bwt_with_libdivsufsort<P: Position>(
    text: &mut Vec<u8>,
    sampling_ratio: P,
) -> (Vec<P>, P) {
    let suffix_array = divsufsort(text).unwrap();
    let pidx = {
        let mut sa = suffix_array.clone();
        let pidx = bw_transform(text, &mut sa).unwrap();
        pidx
    };

    let compressed_suffix_array = suffix_array.into_iter()
        .step_by(sampling_ratio.as_usize())
        .map(|x| P::from_i64(x))
        .collect();
    (compressed_suffix_array, P::from_i64(pidx))
}
