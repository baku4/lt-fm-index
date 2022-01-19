use crate::core::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};

use crate::composition::{
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex,
    TextType, BwtCompressionSize,
};

struct LtFmIndexBuilder {
    text_type: TextType,
    bwt_compression: BwtCompressionSize,
    suffix_array_sampling_ratio: u64,
    lookup_table_kmer_size: Option<usize>,
}

impl TextType {
    fn recommended_kmer_size(&self) -> usize {
        match self {
            Self::NucleotideOnly => 8, // About 64 Kb for kmer count array
            Self::NucleotideWithNoise => 7, // About 76 Kb for kmer count array
            Self::AminoAcidOnly => 4, // About 156 Kb for kmer count array
            Self::AminoAcidWithNoise => 4, // About 190 Kb for kmer count array
        }
    }
}

impl Default for LtFmIndexBuilder {
    fn default() -> Self {
        Self {
            text_type: TextType::NucleotideOnly,
            bwt_compression: BwtCompressionSize::_64,
            suffix_array_sampling_ratio: 2,
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
    pub fn use_nucleotide_only(mut self) -> Self {
        self.text_type = TextType::NucleotideOnly;
        self
    }
    pub fn use_nucleotide_with_noise(mut self) -> Self {
        self.text_type = TextType::NucleotideWithNoise;
        self
    }
    pub fn use_amino_acid_only(mut self) -> Self {
        self.text_type = TextType::AminoAcidOnly;
        self
    }
    pub fn use_amino_acid_with_noise(mut self) -> Self {
        self.text_type = TextType::AminoAcidWithNoise;
        self
    }
    // Change Bwt compression size
    pub fn compress_bwt_64(mut self) -> Self {
        self.bwt_compression = BwtCompressionSize::_64;
        self
    }
    pub fn compress_bwt_128(mut self) -> Self {
        self.bwt_compression = BwtCompressionSize::_128;
        self
    }
}