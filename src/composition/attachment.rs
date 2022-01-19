use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};

use super::{
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex,
    TextType, BwtCompressionSize,
};

pub trait OptionPrint {
    fn text_type(&self) -> TextType;
    fn bwt_compression_size(&self) -> BwtCompressionSize;
    fn suffix_array_sampling_ratio(&self) -> u64;
    fn lookup_table_kmer_size(&self) -> usize;
    fn supported_utf8_letters(&self) ->  &[u8];
}

impl OptionPrint for SelfDescLtFmIndex {
    fn text_type(&self) -> TextType {
        match self {
            Self::NO64(_) | Self::NO128(_) => TextType::NucleotideOnly,
            Self::NN64(_) | Self::NN128(_) => TextType::NucleotideWithNoise,
            Self::AO64(_) | Self::AO128(_) => TextType::AminoAcidOnly,
            Self::AN64(_) | Self::AN128(_) => TextType::AminoAcidWithNoise,
        }
    }
    fn bwt_compression_size(&self) -> BwtCompressionSize {
        match self {
            Self::NO64(_) | Self::NN64(_) | Self::AO64(_) | Self::AN64(_) => BwtCompressionSize::_64,
            Self::NO128(_) | Self::NN128(_) | Self::AO128(_) | Self::AN128(_) => BwtCompressionSize::_128,
        }
    }
    fn suffix_array_sampling_ratio(&self) -> u64 {
        match self {
            Self::NO64(inner) => inner.suffix_array_sampling_ratio(),
            Self::NO128(inner) => inner.suffix_array_sampling_ratio(),
            Self::NN64(inner) => inner.suffix_array_sampling_ratio(),
            Self::NN128(inner) => inner.suffix_array_sampling_ratio(),
            Self::AO64(inner) => inner.suffix_array_sampling_ratio(),
            Self::AO128(inner) => inner.suffix_array_sampling_ratio(),
            Self::AN64(inner) => inner.suffix_array_sampling_ratio(),
            Self::AN128(inner) => inner.suffix_array_sampling_ratio(),
        }
    }
    fn lookup_table_kmer_size(&self) -> usize {
        match self {
            Self::NO64(inner) => inner.lookup_table_kmer_size(),
            Self::NO128(inner) => inner.lookup_table_kmer_size(),
            Self::NN64(inner) => inner.lookup_table_kmer_size(),
            Self::NN128(inner) => inner.lookup_table_kmer_size(),
            Self::AO64(inner) => inner.lookup_table_kmer_size(),
            Self::AO128(inner) => inner.lookup_table_kmer_size(),
            Self::AN64(inner) => inner.lookup_table_kmer_size(),
            Self::AN128(inner) => inner.lookup_table_kmer_size(),
        }
    }
    fn supported_utf8_letters(&self) -> &[u8] {
        match self {
            Self::NO64(_) | Self::NO128(_) => b"ACGT",
            Self::NN64(_) | Self::NN128(_) => b"ACGT_",
            Self::AO64(_) | Self::AO128(_) => b"ACDEFGHIKLMNPQRSTVWY",
            Self::AN64(_) | Self::AN128(_) => b"ACDEFGHIKLMNPQRSTVWY_",
        }
    }
}
