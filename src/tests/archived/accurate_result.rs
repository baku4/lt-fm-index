use crate::*;
use crate::tests::random_text::*;
use crate::tests::other_crate::*;

#[test]
fn test_all_types_of_structures_are_accurate() {
    let kmer_size = 4;
    let sa_sampling_ratio = 4;

    let text_count = 10;
    let pattern_count_for_each_text = 5;

    for c in 0..text_count {
        println!("Text count: {}/{}", c+1, text_count);
        // NO
        {
            let text = rand_text_of_no();
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_nucleotide_only()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_nucleotide_only()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());

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
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_nucleotide_with_noise()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_nucleotide_with_noise()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());

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
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_amino_acid_only()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_amino_acid_only()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());

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
            let lt_fm_index_64 = LtFmIndexBuilder::new()
                .use_amino_acid_with_noise()
                .compress_bwt_64()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());
            let lt_fm_index_128 = LtFmIndexBuilder::new()
                .use_amino_acid_with_noise()
                .compress_bwt_128()
                .set_lookup_table_kmer_size(kmer_size).unwrap()
                .set_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
                .build(text.clone());

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
