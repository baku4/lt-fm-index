use super::{
    Result, error_msg,
    FmIndex, Pattern, Text, TextType, BitSize,
};

pub struct FmIndexConfig {
    /// Type of text
    text_type: TextType,
    /// Kmer size for kmer lookup table
    kmer_size: usize,
    /// Bit size for bit count lookup table
    bit_size: BitSize,
    /// Sampling ratio of suffix array
    sa_sampling_ratio: u64,
}

impl FmIndexConfig {
    pub fn new_only_nucleotide() -> Self {
        Self::new_with_text_type(TextType::OnlyNucleotide)
    }
    pub fn new_nucleotide_with_noise() -> Self {
        Self::new_with_text_type(TextType::NucleotideWithNoise)
    }
    pub fn new_only_aminoacid() -> Self {
        Self::new_with_text_type(TextType::OnlyAminoacid)
    }
    pub fn new_aminoacid_with_noise() -> Self {
        Self::new_with_text_type(TextType::AminoacidWithNoise)
    }
    fn new_with_text_type(text_type: TextType) -> Self {
        let kmer_size = text_type.default_kmer_size();
        let bit_size = BitSize::default();
        let sa_sampling_ratio = DEFAULT_SA_SAMPLING_RATIO;
        Self {
            text_type,
            kmer_size,
            bit_size,
            sa_sampling_ratio,
        }
    }
    
    pub fn change_kmer_size(mut self, kmer_size: usize) -> Result<Self> {
        let pointer_width: usize = Self::pointer_width_of_target();
        let max_kmer = pointer_width/2;
        
        if kmer_size < 2 {
            error_msg!("The size of the kmer cannot be less than 2")
        } else if kmer_size > max_kmer {
            error_msg!("The size of the kmer cannot be greater than {} which is limited to half of pointer width({} bits) of target system", max_kmer, pointer_width);
        } else {
            self.kmer_size = kmer_size;
            Ok(self)
        }
    }
    pub fn change_bit_size_to_8(mut self) -> Self {
        self.bit_size = BitSize::Bit8;
        self
    }
    pub fn change_bit_size_to_16(mut self) -> Self {
        self.bit_size = BitSize::Bit16;
        self
    }
    pub fn change_sa_sampling_ratio(mut self, sa_sampling_ratio: u64) -> Result<Self> {
        if sa_sampling_ratio < 1 {
            error_msg!("The sampling ratio allows only positive integer");
        } else {
            self.sa_sampling_ratio = sa_sampling_ratio;
            Ok(self)
        }
    }

    pub fn generate(&self, text: Text) {
        // TODO:
    }

    fn pointer_width_of_target() -> usize {
        #[cfg(target_pointer_width = "32")]
        let pointer_width: usize = 32;
        #[cfg(target_pointer_width = "64")]
        let pointer_width: usize = 64;
        pointer_width
    }
}

impl TextType {
    fn default_kmer_size(&self) -> usize {
        match self {
            Self::OnlyNucleotide => 12, // 4^12 = 16,777,216 bytes
            Self::NucleotideWithNoise => 10, // 5^10 = 9,765,625 bytes
            Self::OnlyAminoacid => 5, // 20^5 = 3,200,000 bytes
            Self::AminoacidWithNoise => 5, // 21^5 = 4,084,101 bytes
        }
    }
}

impl Default for BitSize {
    fn default() -> Self {
        Self::Bit8
    }
}

const DEFAULT_SA_SAMPLING_RATIO: u64 = 2;