use crate::{FmIndex, Result, error_msg};
use crate::{Text};

use super::use_case::*;

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: usize = 64;

#[derive(Debug)]
pub struct LtFmIndexConfig {
    /// Type of text
    text_type: TextType,
    /// Kmer size for lookup table
    kmer_size: Option<usize>,
    /// Sampling ratio of suffix array
    sa_sampling_ratio: u64,
    /// Bwt interval of rank count
    bwt_interval: BwtInterval,
}

impl LtFmIndexConfig {
    pub fn for_nucleotide() -> Self {
        Self {
            text_type: TextType::NucleotideOnly,
            kmer_size: None,
            sa_sampling_ratio: Self::default_sa_sampling_ratio(),
            bwt_interval: Self::default_bwt_interval(),
        }
    }
    pub fn for_aminoacid() -> Self {
        Self {
            text_type: TextType::AminoacidOnly,
            kmer_size: None,
            sa_sampling_ratio: Self::default_sa_sampling_ratio(),
            bwt_interval: Self::default_bwt_interval(),
        }
    }
    pub fn with_noise(mut self) -> Self {
        match self.text_type {
            TextType::NucleotideOnly => {
                self.text_type = TextType::NucleotideWithNoise;
            },
            TextType::AminoacidOnly => {
                self.text_type = TextType::AminoacidWithNoise;
            },
            _ => {},
        }
        self
    }
    pub fn change_kmer_size(mut self, kmer_size: usize) -> Result<Self> {
        let max_kmer = POINTER_WIDTH / 2;
        
        if kmer_size < 2 {
            error_msg!("The size of the kmer cannot be less than 2")
        } else if kmer_size > max_kmer {
            error_msg!("The size of the kmer cannot be greater than {} which is limited to half of pointer width({} bits) of target system", max_kmer, POINTER_WIDTH);
        } else {
            self.kmer_size = Some(kmer_size);
            Ok(self)
        }
    }
    pub fn change_suffix_array_sampling_ratio(mut self, sa_sampling_ratio: u64) -> Result<Self> {
        if sa_sampling_ratio < 1 {
            error_msg!("The sampling ratio allows only positive integer");
        } else {
            self.sa_sampling_ratio = sa_sampling_ratio;
            Ok(self)
        }
    }
    pub fn change_bwt_interval_to_128(mut self) -> Self {
        self.bwt_interval = BwtInterval::_128;
        self
    }
    // TODO: can generate with text larger than kmer size
    pub fn generate(self, text: Text) -> Box<dyn FmIndex> {
        match self.text_type {
            TextType::NucleotideOnly => {
                match self.bwt_interval {
                    BwtInterval::_64 => {
                        Box::new(LtFmIndexNO64::new(text, self.sa_sampling_ratio, self.get_kmer_size()))
                    },
                    BwtInterval::_128 => {
                        Box::new(LtFmIndexNO128::new(text, self.sa_sampling_ratio, self.get_kmer_size()))
                    },
                }
            },
            TextType::NucleotideWithNoise => {
                match self.bwt_interval {
                    BwtInterval::_64 => {
                        Box::new(LtFmIndexNN64::new(text, self.sa_sampling_ratio, self.get_kmer_size()))
                    },
                    BwtInterval::_128 => {
                        Box::new(LtFmIndexNN128::new(text, self.sa_sampling_ratio, self.get_kmer_size()))
                    },
                }
            },
            TextType::AminoacidOnly => {
                match self.bwt_interval {
                    BwtInterval::_64 => {
                        Box::new(LtFmIndexAO64::new(text, self.sa_sampling_ratio, self.get_kmer_size()))
                    },
                    BwtInterval::_128 => {
                        Box::new(LtFmIndexAO128::new(text, self.sa_sampling_ratio, self.get_kmer_size()))
                    },
                }
            },
            TextType::AminoacidWithNoise => {
                match self.bwt_interval {
                    BwtInterval::_64 => {
                        Box::new(LtFmIndexAN64::new(text, self.sa_sampling_ratio, self.get_kmer_size()))
                    },
                    BwtInterval::_128 => {
                        Box::new(LtFmIndexAN128::new(text, self.sa_sampling_ratio, self.get_kmer_size()))
                    },
                }
            },
        }
    }

    fn get_kmer_size(&self) -> usize {
        match self.kmer_size {
            Some(kmer_size) => kmer_size,
            None => self.text_type.default_kmer_size(),
        }
    }
    fn default_sa_sampling_ratio() -> u64 {
        2
    }
    fn default_bwt_interval() -> BwtInterval {
        BwtInterval::_64
    }
}

#[derive(Debug)]
enum TextType {
    NucleotideOnly,
    NucleotideWithNoise,
    AminoacidOnly,
    AminoacidWithNoise,
}

impl TextType {
    fn default_kmer_size(&self) -> usize {
        match self {
            Self::NucleotideOnly => 10, // 5^10 * 8 = 78,125,000 bytes
            Self::NucleotideWithNoise => 9, // 6^9 * 8 = 80,621,568 bytes
            Self::AminoacidOnly => 5, // 21^5 * 8 = 32,672,808 bytes
            Self::AminoacidWithNoise => 5, // 22^5 * 8 = 41,229,056 bytes
        }
    }
}

#[derive(Debug)]
enum BwtInterval {
    _64,
    _128,
}
