use super::{
    LtFmIndex,
    TextType,
    BwtBlockSize,
};

mod build;
mod change_options;
mod features;

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: usize = 64;

#[derive(Debug, Clone)]
pub struct LtFmIndexBuilder {
    text_type: Option<TextType>,
    bwt_block_size: BwtBlockSize,
    suffix_array_sampling_ratio: u64,
    lookup_table_kmer_size: Option<usize>,
}

use thiserror::Error;
#[derive(Error, Debug)]
pub enum BuildError {
    #[error("The type of text can not be inferred.")]
    TextTypeError,
    #[error("Suffix array sampling ratio allows a value >= 1")]
    SasrBound,
    #[error("Lookup table kmer size allows a value >= 2 and <= half the width of pointer")]
    LtksBound,
}

const DEFAULT_BBS: BwtBlockSize = BwtBlockSize::_128;
const DEFAULT_SASR: u64 = 2;
impl Default for LtFmIndexBuilder {
    fn default() -> Self {
        Self {
            text_type: None,
            bwt_block_size: DEFAULT_BBS,
            suffix_array_sampling_ratio: DEFAULT_SASR,
            lookup_table_kmer_size: None,
        }
    }
}

impl LtFmIndexBuilder {
    // New builder with default option
    pub fn new() -> Self {
        Default::default()
    }
    // Change text type
    pub fn text_type_is_inferred(mut self) -> Self {
        self.text_type = None;
        self
    }
    pub fn text_type_is_nucleotide_only(mut self) -> Self {
        self.text_type = Some(TextType::NucleotideOnly);
        self
    }
    pub fn text_type_is_nucleotide_with_noise(mut self) -> Self {
        self.text_type = Some(TextType::NucleotideWithNoise);
        self
    }
    pub fn text_type_is_amino_acid_only(mut self) -> Self {
        self.text_type = Some(TextType::AminoAcidOnly);
        self
    }
    pub fn text_type_is_amino_acid_with_noise(mut self) -> Self {
        self.text_type = Some(TextType::AminoAcidWithNoise);
        self
    }
    // Change BWT block size
    pub fn bwt_block_size_is_default(mut self) -> Self {
        self.bwt_block_size = DEFAULT_BBS;
        self
    }
    pub fn bwt_block_size_is_64(mut self) -> Self {
        self.bwt_block_size = BwtBlockSize::_64;
        self
    }
    pub fn bwt_block_size_is_128(mut self) -> Self {
        self.bwt_block_size = BwtBlockSize::_128;
        self
    }
    // Change suffix array sampling ratio
    pub fn set_suffix_array_sampling_ratio_to_default(mut self) -> Self {
        self.suffix_array_sampling_ratio = DEFAULT_SASR;
        self
    }
    pub fn set_suffix_array_sampling_ratio(
        mut self,
        sampling_ratio: u64,
    ) -> Result<Self, BuildError> {
        if sampling_ratio > 0 {
            self.suffix_array_sampling_ratio = sampling_ratio;
            Ok(self)
        } else {
            Err(BuildError::SasrBound)
        }
    }
    // Set lookup table kmer size
    pub fn set_lookup_table_kmer_size_to_default(mut self) -> Self {
        self.lookup_table_kmer_size = None;
        self
    }
    pub fn set_lookup_table_kmer_size(
        mut self,
        kmer_size: usize,
    ) -> Result<Self, BuildError> {
        let max_kmer = POINTER_WIDTH / 2;
        
        if kmer_size < 2 || kmer_size > max_kmer {
            Err(BuildError::LtksBound)
        } else {
            self.lookup_table_kmer_size = Some(kmer_size);
            Ok(self)
        }
    }
}
