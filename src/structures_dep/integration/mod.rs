use crate::core::{
    LtFmIndexInterface, FmIndexInterface,
};
use super::{
    LtFmIndex64NO, LtFmIndex128NO, LtFmIndex64NN, LtFmIndex128NN,
    LtFmIndex64AO, LtFmIndex128AO, LtFmIndex64AN, LtFmIndex128AN,
};

mod features;
pub use features::IoError;

/// The index to (1) count or (2) locate the pattern in the indexed text.
#[derive(Clone, PartialEq, Eq)]
pub struct LtFmIndexDep {
    inner_wrapper: InnerWrapper,
}
// Self-descriptive structure wrapper
// - Size
//   - All use-cases are 152 bytes
//   - InnerWrapper is 160 bytes
#[derive(Debug, Clone, PartialEq, Eq)]
enum InnerWrapper {
    NO64(LtFmIndex64NO),
    NO128(LtFmIndex128NO),
    NN64(LtFmIndex64NN),
    NN128(LtFmIndex128NN),
    AO64(LtFmIndex64AO),
    AO128(LtFmIndex128AO),
    AN64(LtFmIndex64AN),
    AN128(LtFmIndex128AN),
}
/// Text type marker
///   - NucleotideOnly
///      - ACG and wildcard
///   - NucleotideWithNoise
///      - ACGT and wildcard
///   - AminoAcidOnly
///      - ACDEFGHIKLMNPQRSTVW and wildcard
///   - AminoAcidWithNoise
///      - ACDEFGHIKLMNPQRSTVWY and wildcard
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextTypeDep {
    NucleotideOnly,
    NucleotideWithNoise,
    AminoAcidOnly,
    AminoAcidWithNoise,
}
/// Bwt block size marker
/// Using the larger block size makes the index small but slightly slow.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BwtBlockSizeDep {
    _64,
    _128,
}

impl LtFmIndexDep {
    /// Generally, LtFmIndex is assumed to be created using [LtFmIndexBuilder].
    /// This raw method is slightly faster, but using builder is safe and concise.
    pub fn new(
        text: Vec<u8>,
        suffix_array_sampling_ratio: u64,
        lookup_table_kmer_size: usize,
        text_type: TextTypeDep,
        bwt_block_size: BwtBlockSizeDep,
    ) -> Self {
        let inner_wrapper = match text_type {
            TextTypeDep::NucleotideOnly => {
                match bwt_block_size {
                    BwtBlockSizeDep::_64 => {
                        InnerWrapper::NO64(
                            LtFmIndex64NO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtBlockSizeDep::_128 => {
                        InnerWrapper::NO128(
                            LtFmIndex128NO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
            TextTypeDep::NucleotideWithNoise => {
                match bwt_block_size {
                    BwtBlockSizeDep::_64 => {
                        InnerWrapper::NN64(
                            LtFmIndex64NN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtBlockSizeDep::_128 => {
                        InnerWrapper::NN128(
                            LtFmIndex128NN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
            TextTypeDep::AminoAcidOnly => {
                match bwt_block_size {
                    BwtBlockSizeDep::_64 => {
                        InnerWrapper::AO64(
                            LtFmIndex64AO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtBlockSizeDep::_128 => {
                        InnerWrapper::AO128(
                            LtFmIndex128AO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
            TextTypeDep::AminoAcidWithNoise => {
                match bwt_block_size {
                    BwtBlockSizeDep::_64 => {
                        InnerWrapper::AN64(
                            LtFmIndex64AN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtBlockSizeDep::_128 => {
                        InnerWrapper::AN128(
                            LtFmIndex128AN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
        };

        Self { inner_wrapper }
    }
}

impl LtFmIndexDep {
    /// Count the pattern in the indexed text
    #[inline]
    pub fn count(&self, pattern: &[u8]) -> u64 {
        match &self.inner_wrapper {
            InnerWrapper::NO64(inner) => inner.count(pattern),
            InnerWrapper::NO128(inner) => inner.count(pattern),
            InnerWrapper::NN64(inner) => inner.count(pattern),
            InnerWrapper::NN128(inner) => inner.count(pattern),
            InnerWrapper::AO64(inner) => inner.count(pattern),
            InnerWrapper::AO128(inner) => inner.count(pattern),
            InnerWrapper::AN64(inner) => inner.count(pattern),
            InnerWrapper::AN128(inner) => inner.count(pattern),
        }
    }
    /// Locate the pattern in the indexed text
    #[inline]
    pub fn locate(&self, pattern: &[u8]) -> Vec<u64> {
        match &self.inner_wrapper {
            InnerWrapper::NO64(inner) => inner.locate(pattern),
            InnerWrapper::NO128(inner) => inner.locate(pattern),
            InnerWrapper::NN64(inner) => inner.locate(pattern),
            InnerWrapper::NN128(inner) => inner.locate(pattern),
            InnerWrapper::AO64(inner) => inner.locate(pattern),
            InnerWrapper::AO128(inner) => inner.locate(pattern),
            InnerWrapper::AN64(inner) => inner.locate(pattern),
            InnerWrapper::AN128(inner) => inner.locate(pattern),
        }
    }
}
