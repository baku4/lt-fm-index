use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex,
    TextType, BwtCompressionSize,
    LtFmIndex,
};

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: usize = 64;

#[derive(Debug, Clone)]
pub struct LtFmIndexBuilder {
    text_type: TextType,
    bwt_compression: BwtCompressionSize,
    suffix_array_sampling_ratio: u64,
    lookup_table_kmer_size: Option<usize>,
}

impl TextType {
    pub fn recommended_kmer_size(&self) -> usize {
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
    // Build LtFmIndex
    pub fn build(self, text: Text) -> LtFmIndex {
        let lookup_table_kmer_size = match self.lookup_table_kmer_size {
            Some(value) => value,
            None => self.text_type.recommended_kmer_size(),
        };

        let self_desc_lt_fm_index_pre_build = SelfDescLtFmIndexPreBuild::new(
            text,
            self.suffix_array_sampling_ratio,
            lookup_table_kmer_size,
            self.text_type,
            self.bwt_compression,
        );

        LtFmIndex::new_from_bytes_unchecked(self_desc_lt_fm_index_pre_build.encode_to_bytes())
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
    // Change suffix array sampling ratio
    pub fn set_suffix_array_sampling_ratio(
        mut self,
        sr: u64,
    ) -> Result<Self> {
        if sr > 0 {
            self.suffix_array_sampling_ratio = sr;
            Ok(self)
        } else {
            error_msg!("Sampling ratio accept positive integer.")
        }
    }
    // Set lookup table kmer size
    pub fn set_lookup_table_kmer_size(
        mut self,
        kmer_size: usize,
    ) -> Result<Self> {
        let max_kmer = POINTER_WIDTH / 2;
        
        if kmer_size < 2 {
            error_msg!("The size of the kmer cannot be less than 2")
        } else if kmer_size > max_kmer {
            error_msg!("The size of the kmer cannot be greater than {} which is limited to half of pointer width({} bits) of target system", max_kmer, POINTER_WIDTH);
        } else {
            self.lookup_table_kmer_size = Some(kmer_size);
            Ok(self)
        }
    }
}
