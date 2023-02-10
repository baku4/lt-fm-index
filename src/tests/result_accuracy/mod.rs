use crate::*;
use crate::tests::random_text::*;
use crate::tests::other_crate::*;

use crate::structures::*;

#[test]
fn results_are_accurate() {
    let kmer_size = 4;
    let sa_sampling_ratio = 4;

    let text_count = 10;
    let pattern_count_for_each_text = 5;

    for c in 0..text_count {

    }
}

#[test]
fn new_struct_is_accurate() {
    let kmer_size = 4;
    let sa_sampling_ratio = 4;

    let text_count = 20;
    let pattern_count_for_each_text = 5;

    for c in 0..text_count {
        println!("Text count: {}/{}", c+1, text_count);
        {
            let text = rand_text_of_no();
            let text_encoder = text_encoders::C3B64::new(&[
                &vec![b'A'],
                &vec![b'C'],
                &vec![b'G'],
            ]);
            let new_lt_fm_index = LtFmIndex::new(
                text.clone(),
                &text_encoder,
                sa_sampling_ratio,
                kmer_size as u32,
            );

            let old_lt_fm_index = LtFmIndexBuilderDep::new()
                .text_type_is_nucleotide_only()
                .bwt_block_size_is_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone()).unwrap();

            let fm_index_other = get_fmindex_of_other_crate(&text);

            for _ in 0..pattern_count_for_each_text {
                let pattern = rand_pattern_of_text(&text);
                let answer = get_sorted_locations(&fm_index_other, &pattern);

                let mut res_new = new_lt_fm_index.locate(&pattern);
                let mut res_old = old_lt_fm_index.locate(&pattern);
                res_new.sort();
                res_old.sort();

                assert_eq!(res_new, res_old);
                assert_eq!(res_new, answer);
            }
        }
    }
}

#[test]
fn results_are_accurate() {
    let kmer_size = 4;
    let sa_sampling_ratio = 4;

    let text_count = 10;
    let pattern_count_for_each_text = 5;

    for c in 0..text_count {
        println!("Text count: {}/{}", c+1, text_count);
        // NO
        {
            let text = rand_text_of_no();
            let lt_fm_index_64 = LtFmIndexBuilderDep::new()
                .text_type_is_nucleotide_only()
                .bwt_block_size_is_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone()).unwrap();
            let lt_fm_index_128 = LtFmIndexBuilderDep::new()
                .text_type_is_nucleotide_only()
                .bwt_block_size_is_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone()).unwrap();

            let fm_index_other = get_fmindex_of_other_crate(&text);

            for _ in 0..pattern_count_for_each_text {
                let pattern = rand_pattern_of_text(&text);
                let answer = get_sorted_locations(&fm_index_other, &pattern);

                let mut res_64 = lt_fm_index_64.locate(&pattern);
                res_64.sort();
                let mut res_128 = lt_fm_index_128.locate(&pattern);
                res_128.sort();

                assert_eq!(res_64, answer);
                assert_eq!(res_128, answer);
            }
        }
        // NN
        {
            let text = rand_text_of_nn();
            let lt_fm_index_64 = LtFmIndexBuilderDep::new()
                .text_type_is_nucleotide_with_noise()
                .bwt_block_size_is_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone()).unwrap();
            let lt_fm_index_128 = LtFmIndexBuilderDep::new()
                .text_type_is_nucleotide_with_noise()
                .bwt_block_size_is_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone()).unwrap();

            let fm_index_other = get_fmindex_of_other_crate(&text);

            for _ in 0..pattern_count_for_each_text {
                let pattern = rand_pattern_of_text(&text);
                let answer = get_sorted_locations(&fm_index_other, &pattern);

                let mut res_64 = lt_fm_index_64.locate(&pattern);
                res_64.sort();
                let mut res_128 = lt_fm_index_128.locate(&pattern);
                res_128.sort();

                assert_eq!(res_64, answer);
                assert_eq!(res_128, answer);
            }
        }
        // AO
        {
            let text = rand_text_of_ao();
            let lt_fm_index_64 = LtFmIndexBuilderDep::new()
                .text_type_is_amino_acid_only()
                .bwt_block_size_is_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone()).unwrap();
            let lt_fm_index_128 = LtFmIndexBuilderDep::new()
                .text_type_is_amino_acid_only()
                .bwt_block_size_is_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone()).unwrap();

            let fm_index_other = get_fmindex_of_other_crate(&text);

            for _ in 0..pattern_count_for_each_text {
                let pattern = rand_pattern_of_text(&text);
                let answer = get_sorted_locations(&fm_index_other, &pattern);

                let mut res_64 = lt_fm_index_64.locate(&pattern);
                res_64.sort();
                let mut res_128 = lt_fm_index_128.locate(&pattern);
                res_128.sort();

                assert_eq!(res_64, answer);
                assert_eq!(res_128, answer);
            }
        }
        // AN
        {
            let text = rand_text_of_an();
            let lt_fm_index_64 = LtFmIndexBuilderDep::new()
                .text_type_is_amino_acid_with_noise()
                .bwt_block_size_is_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone()).unwrap();
            let lt_fm_index_128 = LtFmIndexBuilderDep::new()
                .text_type_is_amino_acid_with_noise()
                .bwt_block_size_is_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone()).unwrap();

            let fm_index_other = get_fmindex_of_other_crate(&text);

            for _ in 0..pattern_count_for_each_text {
                let pattern = rand_pattern_of_text(&text);
                let answer = get_sorted_locations(&fm_index_other, &pattern);

                let mut res_64 = lt_fm_index_64.locate(&pattern);
                res_64.sort();
                let mut res_128 = lt_fm_index_128.locate(&pattern);
                res_128.sort();

                assert_eq!(res_64, answer);
                assert_eq!(res_128, answer);
            }
        }
    }
}
