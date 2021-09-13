mod io;
mod utils;

pub mod fmindex_on;
pub mod fmindex_nn;

use fmindex_on::FmIndexOn;
use fmindex_nn::FmIndexNn;
pub use io::*;
use serde::{Serialize, Deserialize};

/// Configurations for [FmIndex]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct FmIndexConfigDep {
    /// Kmer size of kmer lookup table
    kmer_size: Option<usize>,
    /// Sampling ratio of suffix array
    sa_sampling_ratio: u64,
    /// Whether text contains only nucleotide sequences (ACGT) or not
    pub only_nucleotide: bool,
}
impl FmIndexConfigDep {
    pub fn new() -> Self {
        Self {
            kmer_size: None,
            sa_sampling_ratio: 2,
            only_nucleotide: true,
        }
    }
    /// Set kmer lookup table  
    /// Allowed k-mer size: [2, (pointer width/2)]
    #[inline]
    pub fn set_kmer_lookup_table(mut self, kmer_size: usize) -> Self {
        #[cfg(target_pointer_width = "32")]
        let pointer_width: usize = 32;
        #[cfg(target_pointer_width = "64")]
        let pointer_width: usize = 64;
        let max_kmer = pointer_width/2;
        // check valid kmer
        if kmer_size < 2 {
            panic!("The size of the kmer cannot be less than 2");
        } else if kmer_size > max_kmer {
            panic!("The size of the kmer cannot be greater than {} which is limited to half of pointer width({} bits) of target system", max_kmer, pointer_width);
        } else {
            self.kmer_size = Some(kmer_size);
            self
        }
    }
    /// Disable kmer lookup table
    #[inline]
    pub fn disable_kmer_lookup_table(mut self) -> Self {
        self.kmer_size = None;
        self
    }
    /// Set sampling ratio of suffix array  
    /// Allowed sampling ratio: positive integer(Z-+)
    #[inline]
    pub fn set_suffix_array_sampling_ratio(mut self, sa_sampling_ratio: u64) -> Self {
        // check valid sa_sampling_ratio
        if sa_sampling_ratio < 1 {
            panic!("The sampling ratio allows only positive integer");
        } else {
            self.sa_sampling_ratio = sa_sampling_ratio;
            self
        }
    }
    /// Whether the text contains only nucleotide sequence or not.
    #[inline]
    pub fn only_nucleotide(mut self, only_nc: bool) -> Self {
        self.only_nucleotide = only_nc;
        self
    }
    /// Text contains only nucleotide sequences. (to be deprecated)
    #[inline]
    pub fn contain_only_nucleotide(mut self) -> Self {
        self.only_nucleotide = true;
        self
    }
    /// Text contains non-nucleotide sequences. (to be deprecated)
    #[inline]
    pub fn contain_non_nucleotide(mut self) -> Self {
        self.only_nucleotide = false;
        self
    }
    /// Generate [FmIndex]
    #[inline]
    pub fn generate_fmindex(&self, text: Vec<u8>) -> FmIndexDep {
        if self.only_nucleotide {
            FmIndexDep::OnlyNc(FmIndexOn::new(self, text))
        } else {
            FmIndexDep::NonNc(FmIndexNn::new(self, text))
        }
    }
}

/// FmIndex
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum FmIndexDep {
    OnlyNc(FmIndexOn),
    NonNc(FmIndexNn),
}

impl FmIndexDep {
    /// Generate [FmIndex]
    fn new(config: &FmIndexConfigDep, text: Vec<u8>) -> Self {
        if config.only_nucleotide {
            Self::OnlyNc(FmIndexOn::new(config, text))
        } else {
            Self::NonNc(FmIndexNn::new(config, text))
        }
    }
    /// Count of occurrences of pattern
    pub fn count(&self, pattern: &[u8]) -> u64 {
        match self {
            Self::OnlyNc(fm_index) => {
                fm_index.count(pattern)
            },
            Self::NonNc(fm_index) => {
                fm_index.count(pattern)
            },
        }
    }
    /// Locate the pattern without k-mer lookup table
    pub fn locate_wo_klt(&self, pattern: &[u8]) -> Vec<u64> {
        match self {
            Self::OnlyNc(fm_index) => {
                fm_index.locate_wo_klt(pattern)
            },
            Self::NonNc(fm_index) => {
                fm_index.locate_wo_klt(pattern)
            },
        }
    }
    /// Locate the pattern with k-mer lookup table
    pub fn locate_w_klt(&self, pattern: &[u8]) -> Vec<u64> {
        match self {
            Self::OnlyNc(fm_index) => {
                fm_index.locate_w_klt(pattern)
            },
            Self::NonNc(fm_index) => {
                fm_index.locate_w_klt(pattern)
            },
        }
    }
}

/// LtFmIndex Trait
pub trait LtFmIndexDep {
    /// Count of occurrences of pattern
    fn count(&self, pattern: &[u8]) -> u64;
    /// Locate the pattern without k-mer lookup table
    fn locate_wo_klt(&self, pattern: &[u8]) -> Vec<u64>;
    /// Locate the pattern with k-mer lookup table
    fn locate_w_klt(&self, pattern: &[u8]) -> Vec<u64>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::fmindex_on::FmIndexOn;
    use super::fmindex_nn::FmIndexNn;

    // For cross check
    use crate_fm_index::converter::RangeConverter;
    use crate_fm_index::suffix_array::SuffixOrderSampler;
    use crate_fm_index::{BackwardSearchIndex, FMIndex};
    use rand::Rng;

    fn get_locations_using_other_crate(text: &Vec<u8>, pattern: &Vec<u8>) -> Vec<u64> {
        let converter = RangeConverter::new(b' ', b'~');
        let sampler = SuffixOrderSampler::new().level(2);
        let index = FMIndex::new(text.clone(), converter, sampler);
        let search = index.search_backward(pattern);
        search.locate()
    }

    // Set data
    fn text_on() -> Vec<u8> {
        "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec()
    }
    fn pattern_on() -> Vec<Vec<u8>> {
        vec!["A", "C", "G", "T", "TA", "AA", "GGC", "TTAC", "TACCAC", "AAGTGAAA"].iter().map(|x| x.as_bytes().to_vec()).collect()
    }
    fn text_nn() -> Vec<u8> {
        "CTCCGTACACCTGTTTCGTATCGGNNNAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCNNNCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACNNNCGGATGNNNCATAGATTTCCCCATTTTGCGTANNNNNNNNNNNNNNNNNNCCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec()
    }
    fn pattern_nn() -> Vec<Vec<u8>> {
        ["A", "C", "G", "T", "N", "GA", "AA", "GN", "GGC", "TTAC", "TACCAC", "AAGTGAAA"].iter().map(|x| x.as_bytes().to_vec()).collect()
    }
    const CHARS: [u8; 5] = [65, 67, 71, 84, 95];
    fn text_rand_on() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let text_len: usize = rng.gen_range(50..100);
        let mut text: Vec<u8> = Vec::with_capacity(text_len);
        (0..text_len).for_each(|_| text.push(CHARS[rng.gen_range(0..4)]));
        text
    }
    fn text_rand_nn() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let text_len: usize = rng.gen_range(50..100);
        let mut text: Vec<u8> = Vec::with_capacity(text_len);
        (0..text_len).for_each(|_| text.push(CHARS[rng.gen_range(0..5)]));
        text
    }

    // config
    fn config_on(ssa: u64, kmer: usize) -> FmIndexConfigDep {
        let config = FmIndexConfigDep::new()
            .set_suffix_array_sampling_ratio(ssa)
            .set_kmer_lookup_table(kmer);
        config
    }
    fn config_nn(ssa: u64, kmer: usize) -> FmIndexConfigDep {
        let config = FmIndexConfigDep::new()
            .set_suffix_array_sampling_ratio(ssa)
            .set_kmer_lookup_table(kmer)
            .contain_non_nucleotide();
        config
    }
    // test with random seq
    #[test]
    fn test_with_random_text() {
        let ssa = 8;
        let kmer = 4;
        let text_count = 50;
        let pattern_len = 10;
        for _ in 0..text_count {
            let text_on = text_rand_on();
            let text_nn = text_rand_nn();

            let config_on = config_on(ssa, kmer);
            let config_nn = config_nn(ssa, kmer);

            let fmi_on = config_on.generate_fmindex(text_on.clone());
            let fmi_nn = config_nn.generate_fmindex(text_nn.clone());

            for l in 1..=pattern_len {
                let pattern_on = text_on[..l].to_vec();
                let pattern_nn = text_nn[..l].to_vec();

                let mut loc_on_res = fmi_on.locate_w_klt(&pattern_on);
                loc_on_res.sort();
                let mut loc_nn_res = fmi_nn.locate_w_klt(&pattern_nn);
                loc_nn_res.sort();

                let mut loc_on_ans = get_locations_using_other_crate(&text_on, &pattern_on.to_vec());
                loc_on_ans.sort();
                let mut loc_nn_ans = get_locations_using_other_crate(&text_nn, &pattern_nn.to_vec());
                loc_nn_ans.sort();

                assert_eq!(loc_on_res, loc_on_ans);
                assert_eq!(loc_nn_res, loc_nn_ans);
            }
        }
    }

    // FmIndexOn
    #[test]
    fn test_fmindex_on_count() {
        let config = config_on(8, 4);
        let text = text_on();
        let fm_index = config.generate_fmindex(text.clone());
        for pattern in pattern_on() {
            let count_res = fm_index.count(&pattern);
            let count_ans = get_locations_using_other_crate(&text, &pattern.to_vec()).len() as u64;
            assert_eq!(count_res, count_ans);
        };
    }
    #[test]
    fn test_fmindex_on_locate_wo_klt() {
        let config = config_on(8, 4);
        let text = text_on();
        let fm_index = config.generate_fmindex(text.clone());
        for pattern in pattern_on() {
            let mut locations_res = fm_index.locate_wo_klt(&pattern);
            locations_res.sort();
            let mut locations_ans = get_locations_using_other_crate(&text, &pattern.to_vec());
            locations_ans.sort();
            assert_eq!(locations_res, locations_ans);
        };
    }
    #[test]
    fn test_fmindex_on_locate_w_klt() {
        let config = config_on(8, 4);
        let text = text_on();
        let fm_index = config.generate_fmindex(text.clone());
        for pattern in pattern_on() {
            let mut locations_res = fm_index.locate_w_klt(&pattern);
            locations_res.sort();
            let mut locations_ans = get_locations_using_other_crate(&text, &pattern.to_vec());
            locations_ans.sort();
            assert_eq!(locations_res, locations_ans);
        };
    }
    // FmIndexNn
    #[test]
    fn test_fmindex_nn_count() {
        let config = config_nn(8, 4);
        let text = text_nn();
        let fm_index = config.generate_fmindex(text.clone());
        for pattern in pattern_nn() {
            let count_res = fm_index.count(&pattern);
            let count_ans = get_locations_using_other_crate(&text, &pattern.to_vec()).len() as u64;
            assert_eq!(count_res, count_ans);
        };
    }
    #[test]
    fn test_fmindex_nn_locate_wo_klt() {
        let config = config_nn(8, 4);
        let text = text_nn();
        let fm_index = config.generate_fmindex(text.clone());
        for pattern in pattern_nn() {
            let mut locations_res = fm_index.locate_wo_klt(&pattern);
            locations_res.sort();
            let mut locations_ans = get_locations_using_other_crate(&text, &pattern.to_vec());
            locations_ans.sort();
            assert_eq!(locations_res, locations_ans);
        };
    }
    #[test]
    fn test_fmindex_nn_locate_w_klt() {
        let config = config_nn(8, 4);
        let text = text_nn();
        let fm_index = config.generate_fmindex(text.clone());
        for pattern in pattern_nn() {
            let mut locations_res = fm_index.locate_w_klt(&pattern);
            locations_res.sort();
            let mut locations_ans = get_locations_using_other_crate(&text, &pattern.to_vec());
            locations_ans.sort();
            assert_eq!(locations_res, locations_ans);
        };
    }

    // KLT check
    #[test]
    fn test_klt_is_matched() {
        let ssa = 8;
        let kmer = 3;
        let text = text_on();
        let config_on = config_on(ssa, kmer);
        let config_nn = config_nn(ssa, kmer);
        // klt
        let klt_on = FmIndexOn::new(&config_on, text.clone()).kmer_lookup_table.unwrap().1;
        let klt_nn = FmIndexNn::new(&config_nn, text.clone()).kmer_lookup_table.unwrap().1;
        // truncate
        let mut klt_nn_truncated: Vec<u64> = Vec::new();
        for (idx, v) in klt_nn.iter().enumerate() {
            let mut have_n = false;
            for k in 0..kmer {
                let pow = 5_usize.pow(k as u32);
                let idx_at_position = idx/pow;
                if idx_at_position % 5 == 4 {
                    have_n = true;
                }
            }
            if !have_n {
                klt_nn_truncated.push(*v)
            }
        }

        assert_eq!(klt_nn_truncated, klt_on);
    }

    #[test]
    // for examples
    fn test_examples() {
        // 1. Use [FmIndex] to locate pattern.
        // use lt_fm_index::FmIndexConfig;
        // (1) Define configuration for fm-index
        let fmi_config = FmIndexConfigDep::new()
            .set_kmer_lookup_table(8)
            .set_suffix_array_sampling_ratio(4)
            .contain_non_nucleotide(); // Default is `true`
        
        // (2) Generate fm-index with text
        let text = b"CTCCGTACACCTGTTTCGTATCGGANNN".to_vec();
        let fm_index = fmi_config.generate_fmindex(text); // text is consumed

        // (3) match with pattern
        let pattern = b"TA".to_vec();
        //   - count
        let count = fm_index.count(&pattern);
        assert_eq!(count, 2);
        //   - locate without k-mer lookup table
        let locations = fm_index.locate_wo_klt(&pattern);
        assert_eq!(locations, vec![5,18]);
        //   - locate with k-mer lookup table
        let locations = fm_index.locate_w_klt(&pattern);
        assert_eq!(locations, vec![5,18]);


        // use lt_fm_index::{FmIndexConfig, FmIndex, FmIndexOn, FmIndexNn};
        // (1) Generate `FmIndex`
        let fmi_config = FmIndexConfigDep::new()
            .set_kmer_lookup_table(8)
            .set_suffix_array_sampling_ratio(4);
        let text = b"CTCCGTACACCTGTTTCGTATCGGA".to_vec();
        let fm_index_pre = fmi_config.generate_fmindex(text); // text is consumed

        // (2) Write fm-index to buffer (or file path)
        let mut buffer = Vec::new();
        fm_index_pre.write_index_to(&mut buffer).unwrap();

        // (3) Read fm-index from buffer (or file path)
        let fm_index_pro = FmIndexDep::read_index_from(&buffer[..]).unwrap();

        assert_eq!(fm_index_pre, fm_index_pro);
    }
}
