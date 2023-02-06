use super::Text;
use libdivsufsort_rs::{divsufsort64, bw_transform64};

#[inline]
pub fn get_suffix_array_and_pidx_while_bwt_with_libdivsufsort(text: &mut Text) -> (Vec<i64>, u64) {
    let suffix_array_i64 = divsufsort64(text).unwrap();
    let pidx = {
        let mut sa = suffix_array_i64.clone();
        let pidx = bw_transform64(text, &mut sa).unwrap();
        pidx
    };

    (suffix_array_i64, pidx as u64)
}
