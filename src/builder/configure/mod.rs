use super::{
    TextTypeDep,
    BwtBlockSizeDep,
    LtFmIndexBuilder,
    BuildError,
};

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: usize = 64;

const DEFAULT_BBS: BwtBlockSizeDep = BwtBlockSizeDep::_128;
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
    /// New builder with default option
    pub fn new() -> Self {
        Default::default()
    }
    /// Make the [TextType] to be inferred (This is default option)
    ///  - What is the difference from the specification of TextType manually?
    ///    - If the ACGTY is used as Text, TextType is inferred as TextType::NucleotideWithNoise (ACGT and Y as wildcard)
    ///    - However, when you specify the TextType as TextType::NucleotideOnly, you can force the both T and Y to be treated as wildcard.
    ///       - Locating AATTAA, AAYYAA, AA@@AA, AA**AA, ... all gives the same result.
    pub fn text_type_is_inferred(mut self) -> Self {
        self.text_type = None;
        self
    }
    /// Mark the [TextType] as [TextType::NucleotideOnly]
    pub fn text_type_is_nucleotide_only(mut self) -> Self {
        self.text_type = Some(TextTypeDep::NucleotideOnly);
        self
    }
    /// Mark the [TextType] as [TextType::NucleotideWithNoise]
    pub fn text_type_is_nucleotide_with_noise(mut self) -> Self {
        self.text_type = Some(TextTypeDep::NucleotideWithNoise);
        self
    }
    /// Mark the [TextType] as [TextType::AminoAcidOnly]
    pub fn text_type_is_amino_acid_only(mut self) -> Self {
        self.text_type = Some(TextTypeDep::AminoAcidOnly);
        self
    }
    /// Mark the [TextType] as [TextType::AminoAcidWithNoise]
    pub fn text_type_is_amino_acid_with_noise(mut self) -> Self {
        self.text_type = Some(TextTypeDep::AminoAcidWithNoise);
        self
    }
    /// Use the default BWT block size
    pub fn bwt_block_size_is_default(mut self) -> Self {
        self.bwt_block_size = DEFAULT_BBS;
        self
    }
    /// Use the 64-sized BWT block
    pub fn bwt_block_size_is_64(mut self) -> Self {
        self.bwt_block_size = BwtBlockSizeDep::_64;
        self
    }
    /// Use the 128-sized BWT block
    pub fn bwt_block_size_is_128(mut self) -> Self {
        self.bwt_block_size = BwtBlockSizeDep::_128;
        self
    }
    /// Use the default suffix array sampling ratio
    pub fn set_suffix_array_sampling_ratio_to_default(mut self) -> Self {
        self.suffix_array_sampling_ratio = DEFAULT_SASR;
        self
    }
    /// Set the suffix array sampling ratio
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
    /// Use the default kmer size for lookup table
    pub fn set_lookup_table_kmer_size_to_default(mut self) -> Self {
        self.lookup_table_kmer_size = None;
        self
    }
    /// Set the size of kmer in lookup table
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
