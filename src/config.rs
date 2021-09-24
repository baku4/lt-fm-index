use crate::{Result, error_msg};
use crate::{Text};

use super::use_case::*;

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: usize = 64;

/** Configuration to generate [LtFmIndex] safely.

[LtFmIndexConfig] can safely generate all implementation types in the [crate::use_case] module through limitations of setting values.

- **Default Setting**
  - Text with no noise (NucleotideOnly, AminoacidOnly)
  - Default kmer size
    - NucleotideOnly: **10**
    - NucleotideWithNoise: **9**
    - AminoacidOnly: **5**
    - AminoacidWithNoise: **5**
  - Sampling ratio is **2**
  - Bwt interval is **64** */
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
    /// New Config for nucleotide
    pub fn for_nucleotide() -> Self {
        Self {
            text_type: TextType::NucleotideOnly,
            kmer_size: None,
            sa_sampling_ratio: Self::default_sa_sampling_ratio(),
            bwt_interval: Self::default_bwt_interval(),
        }
    }
    /// New Config for aminoacid
    pub fn for_aminoacid() -> Self {
        Self {
            text_type: TextType::AminoacidOnly,
            kmer_size: None,
            sa_sampling_ratio: Self::default_sa_sampling_ratio(),
            bwt_interval: Self::default_bwt_interval(),
        }
    }
    /// Text contains noise
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
    /// Change kmer size for kmer count table
    ///
    /// Kmer size allows the value not less than the length of the text and half the pointer width.
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
    /// Change sampling ratio for suffix array
    ///
    /// Sampling ratio allows the positive integer
    pub fn change_sampling_ratio(mut self, sa_sampling_ratio: u64) -> Result<Self> {
        if sa_sampling_ratio < 1 {
            error_msg!("The sampling ratio allows only positive integer");
        } else {
            self.sa_sampling_ratio = sa_sampling_ratio;
            Ok(self)
        }
    }
    /// Change bwt interval to **128** (default is **64**)
    pub fn change_bwt_interval_to_128(mut self) -> Self {
        self.bwt_interval = BwtInterval::_128;
        self
    }
    /// Generate `LtFmIndex` with `Text`
    pub fn generate(self, text: Text) -> Result<LtFmIndexAll> {
        let text_len = text.len();
        let kmer_size = match self.kmer_size {
            Some(specified_kmer) => {
                if text_len < specified_kmer {
                    error_msg!(
                        "Text length({}) can't be shorter than kmer size({}).",
                        text_len, specified_kmer,
                    );
                } else {
                    specified_kmer
                }
            },
            None => {
                let default_kmer = self.text_type.default_kmer_size();
                if text_len < default_kmer {
                    text_len
                } else {
                    default_kmer
                }
            },
        };

        Ok(
            match self.text_type {
                TextType::NucleotideOnly => {
                    match self.bwt_interval {
                        BwtInterval::_64 => {
                            LtFmIndexAll::NO64(
                                LtFmIndexNO64::new(text, self.sa_sampling_ratio, kmer_size)
                            )
                        },
                        BwtInterval::_128 => {
                            LtFmIndexAll::NO128(
                                LtFmIndexNO128::new(text, self.sa_sampling_ratio, kmer_size)
                            )
                        },
                    }
                },
                TextType::NucleotideWithNoise => {
                    match self.bwt_interval {
                        BwtInterval::_64 => {
                            LtFmIndexAll::NN64(
                                LtFmIndexNN64::new(text, self.sa_sampling_ratio, kmer_size)
                            )
                        },
                        BwtInterval::_128 => {
                            LtFmIndexAll::NN128(
                                LtFmIndexNN128::new(text, self.sa_sampling_ratio, kmer_size)
                            )
                        },
                    }
                },
                TextType::AminoacidOnly => {
                    match self.bwt_interval {
                        BwtInterval::_64 => {
                            LtFmIndexAll::AO64(
                                LtFmIndexAO64::new(text, self.sa_sampling_ratio, kmer_size)
                            )
                        },
                        BwtInterval::_128 => {
                            LtFmIndexAll::AO128(
                                LtFmIndexAO128::new(text, self.sa_sampling_ratio, kmer_size)
                            )
                        },
                    }
                },
                TextType::AminoacidWithNoise => {
                    match self.bwt_interval {
                        BwtInterval::_64 => {
                            LtFmIndexAll::AN64(
                                LtFmIndexAN64::new(text, self.sa_sampling_ratio, kmer_size)
                            )
                        },
                        BwtInterval::_128 => {
                            LtFmIndexAll::AN128(
                                LtFmIndexAN128::new(text, self.sa_sampling_ratio, kmer_size)
                            )
                        },
                    }
                },
            }
        )
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
