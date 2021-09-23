use crate::*;
use crate::use_case::LtFmIndexWrapper;
use crate::io::IO;
use super::random_text::*;
use super::other_crate::*;

#[test]
fn test_all_use_cases_are_accurate() {
    let kmer_size = 4;
    let sa_sampling_ratio = 4;

    let text_count = 10;
    let pattern_count_for_each_text = 5;

    for c in 0..text_count {
        println!("Text count: {}/{}", c+1, text_count);
        // NO
        {
            let text = rand_text_of_no();
            let lt_fm_index_64 =lt_fm_index_config64_no(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();
            let lt_fm_index_128 =lt_fm_index_config128_no(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();

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
            let lt_fm_index_64 =lt_fm_index_config64_nn(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();
            let lt_fm_index_128 =lt_fm_index_config128_nn(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();

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
            let lt_fm_index_64 =lt_fm_index_config64_ao(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();
            let lt_fm_index_128 =lt_fm_index_config128_ao(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();

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
            let lt_fm_index_64 =lt_fm_index_config64_an(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();
            let lt_fm_index_128 =lt_fm_index_config128_an(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();

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

#[test]
fn test_all_use_cases_are_io_able_to_buffer() {
    let kmer_size = 4;
    let sa_sampling_ratio = 4;

    let text_count = 10;

    for c in 0..text_count {
        println!("Text count: {}/{}", c+1, text_count);
        // NO
        {
            let text = rand_text_of_no();
            let lt_fm_index_64 =lt_fm_index_config64_no(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();
            let lt_fm_index_128 =lt_fm_index_config128_no(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();

            assert_write_to_and_read_from_buffer(lt_fm_index_64);
            assert_write_to_and_read_from_buffer(lt_fm_index_128);
        }
        // NN
        {
            let text = rand_text_of_nn();
            let lt_fm_index_64 =lt_fm_index_config64_nn(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();
            let lt_fm_index_128 =lt_fm_index_config128_nn(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();

            assert_write_to_and_read_from_buffer(lt_fm_index_64);
            assert_write_to_and_read_from_buffer(lt_fm_index_128);
        }
        // AO
        {
            let text = rand_text_of_ao();
            let lt_fm_index_64 =lt_fm_index_config64_ao(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();
            let lt_fm_index_128 =lt_fm_index_config128_ao(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();

            assert_write_to_and_read_from_buffer(lt_fm_index_64);
            assert_write_to_and_read_from_buffer(lt_fm_index_128);
        }
        // AN
        {
            let text = rand_text_of_an();
            let lt_fm_index_64 =lt_fm_index_config64_an(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();
            let lt_fm_index_128 =lt_fm_index_config128_an(kmer_size, sa_sampling_ratio).generate(text.clone()).unwrap();

            assert_write_to_and_read_from_buffer(lt_fm_index_64);
            assert_write_to_and_read_from_buffer(lt_fm_index_128);
        }
    }
}

fn assert_write_to_and_read_from_buffer(lt_fm_index: LtFmIndexWrapper) {
    let mut buffer = Vec::new();
    // write
    lt_fm_index.write_to(&mut buffer).unwrap();
    // read
    let lt_fm_index_from_buffer = LtFmIndexWrapper::read_from(&buffer[..]).unwrap();

    assert_eq!(lt_fm_index, lt_fm_index_from_buffer);
}

fn lt_fm_index_config64_no(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_nucleotide()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
}
fn lt_fm_index_config64_nn(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_nucleotide()
        .with_noise()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
}
fn lt_fm_index_config64_ao(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_aminoacid()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
}
fn lt_fm_index_config64_an(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_aminoacid()
        .with_noise()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
}
fn lt_fm_index_config128_no(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_nucleotide()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
        .change_bwt_interval_to_128()
}
fn lt_fm_index_config128_nn(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_nucleotide()
        .with_noise()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
        .change_bwt_interval_to_128()
}
fn lt_fm_index_config128_ao(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_aminoacid()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
        .change_bwt_interval_to_128()
}
fn lt_fm_index_config128_an(kmer_size: usize, sa_sampling_ratio: u64) -> LtFmIndexConfig {
    LtFmIndexConfig::for_aminoacid()
        .with_noise()
        .change_kmer_size(kmer_size).unwrap()
        .change_suffix_array_sampling_ratio(sa_sampling_ratio).unwrap()
        .change_bwt_interval_to_128()
}