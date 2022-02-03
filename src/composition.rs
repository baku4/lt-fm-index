use crate::core::{
    Result, error_msg,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
    EndianType, ReadBytesExt, WriteBytesExt, Serializable,
};

use crate::structure::{
    LtFmIndex64NO, LtFmIndex128NO, LtFmIndex64NN, LtFmIndex128NN,
    LtFmIndex64AO, LtFmIndex128AO, LtFmIndex64AN, LtFmIndex128AN,
};

// Additional features
mod feature;

// Text type marker
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextType {
    NucleotideOnly,
    NucleotideWithNoise,
    AminoAcidOnly,
    AminoAcidWithNoise,
}

// Bwt compression size marker
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BwtCompressionSize {
    _64,
    _128,
}

// Self-descriptive structure wrapper
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfDescLtFmIndex {
    NO64(LtFmIndex64NO),
    NO128(LtFmIndex128NO),
    NN64(LtFmIndex64NN),
    NN128(LtFmIndex128NN),
    AO64(LtFmIndex64AO),
    AO128(LtFmIndex128AO),
    AN64(LtFmIndex64AN),
    AN128(LtFmIndex128AN),
}

impl SelfDescLtFmIndex {
    pub fn new(
        text: Text,
        suffix_array_sampling_ratio: u64,
        lookup_table_kmer_size: usize,
        text_type: TextType,
        bwt_compression_size: BwtCompressionSize,
    ) -> Self {
        match text_type {
            TextType::NucleotideOnly => {
                match bwt_compression_size {
                    BwtCompressionSize::_64 => {
                        Self::NO64(
                            LtFmIndex64NO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtCompressionSize::_128 => {
                        Self::NO128(
                            LtFmIndex128NO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
            TextType::NucleotideWithNoise => {
                match bwt_compression_size {
                    BwtCompressionSize::_64 => {
                        Self::NN64(
                            LtFmIndex64NN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtCompressionSize::_128 => {
                        Self::NN128(
                            LtFmIndex128NN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
            TextType::AminoAcidOnly => {
                match bwt_compression_size {
                    BwtCompressionSize::_64 => {
                        Self::AO64(
                            LtFmIndex64AO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtCompressionSize::_128 => {
                        Self::AO128(
                            LtFmIndex128AO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
            TextType::AminoAcidWithNoise => {
                match bwt_compression_size {
                    BwtCompressionSize::_64 => {
                        Self::AN64(
                            LtFmIndex64AN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtCompressionSize::_128 => {
                        Self::AN128(
                            LtFmIndex128AN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
        }
    }
}

impl LtFmIndexInterface for SelfDescLtFmIndex {
    #[inline]
    fn count(&self, pattern: Pattern) -> u64 {
        match self {
            Self::NO64(inner) => inner.count(pattern),
            Self::NO128(inner) => inner.count(pattern),
            Self::NN64(inner) => inner.count(pattern),
            Self::NN128(inner) => inner.count(pattern),
            Self::AO64(inner) => inner.count(pattern),
            Self::AO128(inner) => inner.count(pattern),
            Self::AN64(inner) => inner.count(pattern),
            Self::AN128(inner) => inner.count(pattern),
        }
    }
    #[inline]
    fn locate(&self, pattern: Pattern) -> Vec<u64> {
        match self {
            Self::NO64(inner) => inner.locate(pattern),
            Self::NO128(inner) => inner.locate(pattern),
            Self::NN64(inner) => inner.locate(pattern),
            Self::NN128(inner) => inner.locate(pattern),
            Self::AO64(inner) => inner.locate(pattern),
            Self::AO128(inner) => inner.locate(pattern),
            Self::AN64(inner) => inner.locate(pattern),
            Self::AN128(inner) => inner.locate(pattern),
        }
    }
}
