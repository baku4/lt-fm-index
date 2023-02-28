// Type 1: use crate 'bio'
// This is default version
mod crate_bio;
// Type 2: use 'libdivsufsort'
// Faster, but restrict the environment
#[cfg(feature = "fastbwt")]
mod libdivsufsort;

#[cfg(not(feature = "fastbwt"))]
pub use crate_bio::get_compressed_suffix_array_and_pidx_while_bwt_with_crate_bio as get_compressed_suffix_array_and_pidx_while_bwt;
#[cfg(feature = "fastbwt")]
pub use libdivsufsort::get_compressed_suffix_array_and_pidx_while_bwt_with_libdivsufsort as get_compressed_suffix_array_and_pidx_while_bwt;

#[cfg(test)]
#[cfg(feature = "fastbwt")]
mod tests {
    use crate::Position;
    use crate::tests::random_data::*;

    use super::crate_bio::get_compressed_suffix_array_and_pidx_while_bwt_with_crate_bio as bwt1;
    use super::libdivsufsort::get_compressed_suffix_array_and_pidx_while_bwt_with_libdivsufsort as bwt2;

    #[test]
    fn bwt_transform_result() {
        let n_test = 100;
        let min_text_len = 100;
        let max_text_len = 500;
        let chr_counts = 1..5;

        for chr_count in chr_counts {
            println!(" - chr_count: {}", chr_count);
            for n in 0..n_test {
                print!("  - text: {}\r", n);
                let chr_list = gen_rand_chr_list(chr_count);
                let text = gen_rand_text(&chr_list, min_text_len, max_text_len);
                assert_crate_bio_bwt_same_with_libdivsufsort_rs::<u32>(&text);
                assert_crate_bio_bwt_same_with_libdivsufsort_rs::<u64>(&text);
            }
        }
    }
    fn assert_crate_bio_bwt_same_with_libdivsufsort_rs<P: Position>(text: &[u8]) {
        let sampling_ratio_range  = 1..4;
        for sampling_ratio in sampling_ratio_range {
            // Result from crate_bio
            let mut bwt_res_1 = text.to_vec();
            let (suffix_array_1, pidx_1) = bwt1::<P>(
                &mut bwt_res_1,
                P::from_u32(sampling_ratio),
            );

            // Result from libdivsufsort_rs
            let mut bwt_res_2 = text.to_vec();
            let (suffix_array_2, pidx_2) = bwt2::<P>(
                &mut bwt_res_2,
                P::from_u32(sampling_ratio),
            );

            assert_eq!(suffix_array_1, suffix_array_1);
            assert_eq!(bwt_res_1, bwt_res_2);
            assert_eq!(pidx_1, pidx_2);
        }
    }
}
