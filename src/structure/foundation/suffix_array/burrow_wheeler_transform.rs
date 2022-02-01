use super::{Text};

// Type 1: use 'libdivsufsort'
#[cfg(not(target_arch = "wasm32"))]
use libdivsufsort_rs::{divsufsort64, bw_transform64};
#[cfg(not(target_arch = "wasm32"))]
#[inline]
pub fn get_suffix_array_and_pidx_while_bwt_not_for_wasm(text: &mut Text) -> (Vec<i64>, u64) {
    let suffix_array_i64 = divsufsort64(text).unwrap();
    let pidx = {
        let mut sa = suffix_array_i64.clone();
        let pidx = bw_transform64(text, &mut sa).unwrap();
        pidx
    };

    (suffix_array_i64, pidx as u64)
}

// Type 2: use 'crate bio'
// Built-in Burrow Wheeler Transform Function
// For the environment that does not support building `libdivsufsort_rs`
use bio::data_structures::suffix_array::suffix_array as get_suffix_array;
use bio::data_structures::bwt::bwt as get_bwt;

const SENTINEL_SYMBOL: u8 = 0;

#[inline]
pub fn get_suffix_array_and_pidx_while_bwt_for_wasm(text: &mut Text) -> (Vec<i64>, u64) {
    let mut input_string = text.to_vec();
    input_string.push(SENTINEL_SYMBOL);
    let mut suffix_array = get_suffix_array(&input_string);
    let mut bwt = get_bwt(&input_string, &suffix_array);
    
    let pidx = get_pidx_from_bwt(&bwt);

    bwt.remove(pidx);
    suffix_array.remove(0);

    // Change original text to bwt
    *text = bwt;

    (suffix_array.into_iter().map(|v| v as i64).collect(), pidx as u64)
}

fn get_pidx_from_bwt(bwt: &[u8]) -> usize {
    for (index, &character) in bwt.iter().enumerate() {
        if character == SENTINEL_SYMBOL {
            return index
        }
    }
    0
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
        let (suffix_array_answer, pidx_answer) = get_suffix_array_and_pidx_while_bwt_not_for_wasm(&mut bwt_answer);

        // Result from built_in_bwt
        let mut bwt = text.to_vec();
        let (suffix_array, pidx) = get_suffix_array_and_pidx_while_bwt_for_wasm(&mut bwt);
        
        assert_eq!(suffix_array, suffix_array_answer);
        assert_eq!(bwt, bwt_answer);
        assert_eq!(pidx, pidx_answer);
    }
}
