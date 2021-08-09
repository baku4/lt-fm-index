#![allow(dead_code)]
//! # LT FM-Index
//!
//! `lt-fm-index` is library for locate and count nucleotide sequence (ATGC) string.  
//! `lt-fm-index` using k-mer lookup table (As you noticed, LT stands for lookup table).
//! ## Description
//! - Fm-index is a data structure used for pattern matching.
//! - K-mer lookup table(KLT) is precalculated count table containing all kmer occurrences.
//! - With KLT, you can find the first k-mer pattern at once.
//! - Currently, only the genetic sequence (ATGC) can be used.
//! ## Features
//! - Fm-index using KLT with specified k-mer size.
//! - Suffix array compression with sampling ratio.
//! - BWT and suffix array are generated using `libdivsufsort` library.
//! - BWT(burrow wheeler transformed) string and occurrence array (OA) are aligned in one block of 64 strings.
//! - Aligned BWT&OA block encodes 1-byte character in 6-bits.
//! - There are two main functions.
//!     - count: Count the number of patterns in the text
//!     - locate: Locate pattern index in text (KLT can be specified to enable or disable)
//! ## Future works
//! - Input text can be `slice`
//! ## Example
//! FIXME: examples
//! ```rust
//! ```
//! ## Repository
//! [https://github.com/baku4/lt-fm-index](https://github.com/baku4/lt-fm-index)
//! ## Reference
//! - Ferragina, P., et al. (2004). An Alphabet-Friendly FM-Index, Springer Berlin Heidelberg: 150-160.
//! - Anderson, T. and T. J. Wheeler (2021). An optimized FM-index library for nucleotide and amino acid search, Cold Spring Harbor Laboratory.
//! - Wang, Y., X. Li, D. Zang, G. Tan and N. Sun (2018). Accelerating FM-index Search for Genomic Data Processing, ACM.
//! - Yuta Mori. [`libdivsufsort`](https://github.com/y-256/libdivsufsort)

mod io;
mod utils;

pub mod fmindex_on;
pub mod fmindex_nn;

pub use io::*;

/// Configurations for [FmIndex]
pub struct FmIndexConfig {
    /// Kmer size of kmer lookup table
    kmer_size: Option<usize>,
    /// Sampling ratio of suffix array
    sa_sampling_ratio: u64,
    /// Whether text contains only nucleotide sequences (ACGT) or not
    only_nucleotide: bool,
}
impl FmIndexConfig {
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
    /// Text contains only nucleotide sequences.
    #[inline]
    pub fn contain_only_nucleotide(mut self) -> Self {
        self.only_nucleotide = true;
        self
    }
    /// Text contains non-nucleotide sequences.
    #[inline]
    pub fn contain_non_nucleotide(mut self) -> Self {
        self.only_nucleotide = false;
        self
    }
    /// Text contains non-nucleotide sequences.
    #[inline]
    pub fn generate_fmindex(&self, text: Vec<u8>) -> Box<dyn FmIndex> {
        if self.only_nucleotide {
            Box::new(fmindex_on::FmIndexOn::new(self, text))
        } else {
            Box::new(fmindex_nn::FmIndexNn::new(self, text))
        }
    }
}

pub trait FmIndex {
    fn count(&self, pattern: &[u8]) -> u64;
    fn locate_wo_klt(&self, pattern: &[u8]) -> Vec<u64>;
    fn locate_w_klt(&self, pattern: &[u8]) -> Vec<u64>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    use crate::fmindex_on::FmIndexOn;
    use crate::fmindex_nn::FmIndexNn;

    // For cross check
    use fm_index::converter::RangeConverter;
    use fm_index::suffix_array::SuffixOrderSampler;
    use fm_index::{BackwardSearchIndex, FMIndex};
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
        vec!["A", "C", "G", "T", "TA", "AA", "GGC", "TTAC", "TACCAC", "AAGTGAAA"].into_iter().map(|x| x.as_bytes().to_vec()).collect()
    }
    fn text_nn() -> Vec<u8> {
        "CTCCGTACACCTGTTTCGTATCGGNNNAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCNNNCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACNNNCGGATGNNNCATAGATTTCCCCATTTTGCGTANNNNNNNNNNNNNNNNNNCCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec()
    }
    fn pattern_nn() -> Vec<Vec<u8>> {
        ["A", "C", "G", "T", "N", "GA", "AA", "GN", "GGC", "TTAC", "TACCAC", "AAGTGAAA"].into_iter().map(|x| x.as_bytes().to_vec()).collect()
    }
    const chars: [u8; 5] = [65, 67, 71, 84, 95];
    fn text_rand_on() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let text_len: usize = rng.gen_range(50..100);
        let mut text: Vec<u8> = Vec::with_capacity(text_len);
        (0..text_len).for_each(|_| text.push(chars[rng.gen_range(0..4)]));
        text
    }
    fn text_rand_nn() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let text_len: usize = rng.gen_range(50..100);
        let mut text: Vec<u8> = Vec::with_capacity(text_len);
        (0..text_len).for_each(|_| text.push(chars[rng.gen_range(0..5)]));
        text
    }

    // config
    fn config_on(ssa: u64, kmer: usize) -> FmIndexConfig {
        let config = FmIndexConfig::new()
            .set_suffix_array_sampling_ratio(ssa)
            .set_kmer_lookup_table(kmer);
        config
    }
    fn config_nn(ssa: u64, kmer: usize) -> FmIndexConfig {
        let config = FmIndexConfig::new()
            .set_suffix_array_sampling_ratio(ssa)
            .set_kmer_lookup_table(kmer)
            .contain_non_nucleotide();
        config
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
}
