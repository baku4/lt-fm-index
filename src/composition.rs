use crate::core::{
    Result, error_msg,
    Archive, Serialize, Deserialize, CheckBytes,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};

use crate::structure::{
    LtFmIndexPreBuild64NO, LtFmIndexPreBuild128NO, LtFmIndexPreBuild64NN, LtFmIndexPreBuild128NN,
    LtFmIndexPreBuild64AO, LtFmIndexPreBuild128AO, LtFmIndexPreBuild64AN, LtFmIndexPreBuild128AN,
};

// Additional features
mod attachment;
pub use attachment::OptionPrint;
// Serializer
mod serializer;
use serializer::serialize_with_default_serializer;

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
#[derive(Archive, Serialize, Deserialize, Clone)]
#[archive_attr(derive(CheckBytes))]
#[archive(archived = "SelfDescLtFmIndex")]
pub enum SelfDescLtFmIndexPreBuild {
    NO64(LtFmIndexPreBuild64NO),
    NO128(LtFmIndexPreBuild128NO),
    NN64(LtFmIndexPreBuild64NN),
    NN128(LtFmIndexPreBuild128NN),
    AO64(LtFmIndexPreBuild64AO),
    AO128(LtFmIndexPreBuild128AO),
    AN64(LtFmIndexPreBuild64AN),
    AN128(LtFmIndexPreBuild128AN),
}

impl SelfDescLtFmIndexPreBuild {
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
                            LtFmIndexPreBuild64NO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtCompressionSize::_128 => {
                        Self::NO128(
                            LtFmIndexPreBuild128NO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
            TextType::NucleotideWithNoise => {
                match bwt_compression_size {
                    BwtCompressionSize::_64 => {
                        Self::NN64(
                            LtFmIndexPreBuild64NN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtCompressionSize::_128 => {
                        Self::NN128(
                            LtFmIndexPreBuild128NN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
            TextType::AminoAcidOnly => {
                match bwt_compression_size {
                    BwtCompressionSize::_64 => {
                        Self::AO64(
                            LtFmIndexPreBuild64AO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtCompressionSize::_128 => {
                        Self::AO128(
                            LtFmIndexPreBuild128AO::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
            TextType::AminoAcidWithNoise => {
                match bwt_compression_size {
                    BwtCompressionSize::_64 => {
                        Self::AN64(
                            LtFmIndexPreBuild64AN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                    BwtCompressionSize::_128 => {
                        Self::AN128(
                            LtFmIndexPreBuild128AN::new(text, suffix_array_sampling_ratio, lookup_table_kmer_size)
                        )
                    },
                }
            },
        }
    }
    pub fn encode_to_bytes(&self) -> Vec<u8> {
        serialize_with_default_serializer(self)
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
