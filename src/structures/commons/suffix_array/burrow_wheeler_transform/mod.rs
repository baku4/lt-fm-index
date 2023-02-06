use super::{Text};

// Type 1: use crate 'bio'
// This is default version
mod crate_bio;
// Type 2: use 'libdivsufsort'
// Faster, but restrict the environment
#[cfg(feature = "fastbwt")]
mod libdivsufsort;

#[cfg(not(feature = "fastbwt"))]
pub use crate_bio::get_suffix_array_and_pidx_while_bwt_with_crate_bio as get_suffix_array_and_pidx_while_bwt;
#[cfg(feature = "fastbwt")]
pub use libdivsufsort::get_suffix_array_and_pidx_while_bwt_with_libdivsufsort as get_suffix_array_and_pidx_while_bwt;

#[cfg(test)]
#[cfg(feature = "fastbwt")]
mod tests {
    use crate::tests::random_text::*;

    use super::crate_bio::get_suffix_array_and_pidx_while_bwt_with_crate_bio;
    use super::libdivsufsort::get_suffix_array_and_pidx_while_bwt_with_libdivsufsort;

    #[test]
    fn bwt_transform_result() {
        let n_test = 1000;

        for _ in 0..n_test {
            let text_no = rand_text_of_no();
            assert_crate_bio_bwt_same_with_libdivsufsort_rs(&text_no);
            let text_nn = rand_text_of_nn();
            assert_crate_bio_bwt_same_with_libdivsufsort_rs(&text_nn);
            let text_ao = rand_text_of_ao();
            assert_crate_bio_bwt_same_with_libdivsufsort_rs(&text_ao);
            let text_an = rand_text_of_an();
            assert_crate_bio_bwt_same_with_libdivsufsort_rs(&text_an);
        }
    }

    fn assert_crate_bio_bwt_same_with_libdivsufsort_rs(text: &[u8]) {
        // Result from libdivsufsort_rs
        let mut bwt_answer = text.to_vec();
        let (suffix_array_answer, pidx_answer) = get_suffix_array_and_pidx_while_bwt_with_libdivsufsort(&mut bwt_answer);

        // Result from built_in_bwt
        let mut bwt = text.to_vec();
        let (suffix_array, pidx) = get_suffix_array_and_pidx_while_bwt_with_crate_bio(&mut bwt);
        
        assert_eq!(suffix_array, suffix_array_answer);
        assert_eq!(bwt, bwt_answer);
        assert_eq!(pidx, pidx_answer);
    }
}
