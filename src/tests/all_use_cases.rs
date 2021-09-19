use crate::*;
use super::random_text::*;
use super::other_crate::*;

#[test]
fn test_all_use_cases() {
    let kmer_size = 4;
    let sa_sampling_ratio = 1;

    let text_count = 10;
    let pattern_count_for_each_text = 5;

    for c in 0..text_count {
        println!("Text count: {}/{}", c+1, text_count);
        // NO
        {
            let text = rand_text_of_no();
            let lt_fm_index_64 =LtFmIndexConfig64NO(kmer_size, sa_sampling_ratio).generate(text.clone());
            let lt_fm_index_128 =LtFmIndexConfig128NO(kmer_size, sa_sampling_ratio).generate(text.clone());

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
            let lt_fm_index_64 =LtFmIndexConfig64NN(kmer_size, sa_sampling_ratio).generate(text.clone());
            let lt_fm_index_128 =LtFmIndexConfig128NN(kmer_size, sa_sampling_ratio).generate(text.clone());

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
            let lt_fm_index_64 =LtFmIndexConfig64AO(kmer_size, sa_sampling_ratio).generate(text.clone());
            let lt_fm_index_128 =LtFmIndexConfig128AO(kmer_size, sa_sampling_ratio).generate(text.clone());

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
            let lt_fm_index_64 =LtFmIndexConfig64AN(kmer_size, sa_sampling_ratio).generate(text.clone());
            let lt_fm_index_128 =LtFmIndexConfig128AN(kmer_size, sa_sampling_ratio).generate(text.clone());

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

fn LtFmIndexConfig64NO(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_nucleotide()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
}
fn LtFmIndexConfig64NN(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_nucleotide()
        .with_noise()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
}
fn LtFmIndexConfig64AO(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_aminoacid()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
}
fn LtFmIndexConfig64AN(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_aminoacid()
        .with_noise()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
}
fn LtFmIndexConfig128NO(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_nucleotide()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
        .change_bwt_interval_to_128()
}
fn LtFmIndexConfig128NN(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_nucleotide()
        .with_noise()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
        .change_bwt_interval_to_128()
}
fn LtFmIndexConfig128AO(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_aminoacid()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
        .change_bwt_interval_to_128()
}
fn LtFmIndexConfig128AN(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_aminoacid()
        .with_noise()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
        .change_bwt_interval_to_128()
}