use std::fmt::Debug;

use super::{
    LtFmIndex, InnerWrapper, TextType, BwtBlockSize,
};

impl Debug for LtFmIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LtFmIndex")
            .field("text_type", &self.text_type())
            .field("bwt_block_size", &self.bwt_block_size())
            .field("suffix_array_sampling_ratio", &self.suffix_array_sampling_ratio())
            .field("lookup_table_kmer_size", &self.lookup_table_kmer_size())
            .field("indexed_characters", &self.indexed_characters())
            .finish()
    }
}

impl LtFmIndex {
    pub fn text_type(&self) -> TextType {
        match &self.inner_wrapper {
            InnerWrapper::NO64(_) | InnerWrapper::NO128(_) => TextType::NucleotideOnly,
            InnerWrapper::NN64(_) | InnerWrapper::NN128(_) => TextType::NucleotideWithNoise,
            InnerWrapper::AO64(_) | InnerWrapper::AO128(_) => TextType::AminoAcidOnly,
            InnerWrapper::AN64(_) | InnerWrapper::AN128(_) => TextType::AminoAcidWithNoise,
        }
    }
    pub fn bwt_block_size(&self) -> BwtBlockSize {
        match &self.inner_wrapper {
            InnerWrapper::NO64(_) | InnerWrapper::NN64(_) | InnerWrapper::AO64(_) | InnerWrapper::AN64(_) => BwtBlockSize::_64,
            InnerWrapper::NO128(_) | InnerWrapper::NN128(_) | InnerWrapper::AO128(_) | InnerWrapper::AN128(_) => BwtBlockSize::_128,
        }
    }
    pub fn suffix_array_sampling_ratio(&self) -> u64 {
        match &self.inner_wrapper {
            InnerWrapper::NO64(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            InnerWrapper::NO128(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            InnerWrapper::NN64(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            InnerWrapper::NN128(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            InnerWrapper::AO64(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            InnerWrapper::AO128(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            InnerWrapper::AN64(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            InnerWrapper::AN128(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
        }
    }
    pub fn lookup_table_kmer_size(&self) -> usize {
        match &self.inner_wrapper {
            InnerWrapper::NO64(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            InnerWrapper::NO128(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            InnerWrapper::NN64(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            InnerWrapper::NN128(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            InnerWrapper::AO64(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            InnerWrapper::AO128(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            InnerWrapper::AN64(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            InnerWrapper::AN128(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
        }
    }
    pub fn indexed_characters(&self) ->  &[u8] {
        match &self.inner_wrapper {
            InnerWrapper::NO64(_) | InnerWrapper::NO128(_) => b"ACG*",
            InnerWrapper::NN64(_) | InnerWrapper::NN128(_) => b"ACGT*",
            InnerWrapper::AO64(_) | InnerWrapper::AO128(_) => b"ACDEFGHIKLMNPQRSTVW*",
            InnerWrapper::AN64(_) | InnerWrapper::AN128(_) => b"ACDEFGHIKLMNPQRSTVWY*",
        }
    }
}
