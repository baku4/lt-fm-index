use super::{
    Result, error_msg,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
    EndianType, ReadBytesExt, WriteBytesExt, Serializable,
};
use super::{
    LtFmIndex64NO, LtFmIndex128NO, LtFmIndex64NN, LtFmIndex128NN,
    LtFmIndex64AO, LtFmIndex128AO, LtFmIndex64AN, LtFmIndex128AN,
};
use super::{
    SelfDescLtFmIndex, TextType, BwtCompressionSize,
};


impl SelfDescLtFmIndex {
    pub fn text_type(&self) -> TextType {
        match self {
            Self::NO64(_) | Self::NO128(_) => TextType::NucleotideOnly,
            Self::NN64(_) | Self::NN128(_) => TextType::NucleotideWithNoise,
            Self::AO64(_) | Self::AO128(_) => TextType::AminoAcidOnly,
            Self::AN64(_) | Self::AN128(_) => TextType::AminoAcidWithNoise,
        }
    }
    pub fn bwt_compression_size(&self) -> BwtCompressionSize {
        match self {
            Self::NO64(_) | Self::NN64(_) | Self::AO64(_) | Self::AN64(_) => BwtCompressionSize::_64,
            Self::NO128(_) | Self::NN128(_) | Self::AO128(_) | Self::AN128(_) => BwtCompressionSize::_128,
        }
    }
    pub fn suffix_array_sampling_ratio(&self) -> u64 {
        match self {
            Self::NO64(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            Self::NO128(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            Self::NN64(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            Self::NN128(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            Self::AO64(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            Self::AO128(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            Self::AN64(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
            Self::AN128(raw_lt_fm_index) => {
                raw_lt_fm_index.suffix_array_sampling_ratio()
            },
        }
    }
    pub fn lookup_table_kmer_size(&self) -> usize {
        match self {
            Self::NO64(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            Self::NO128(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            Self::NN64(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            Self::NN128(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            Self::AO64(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            Self::AO128(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            Self::AN64(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
            Self::AN128(raw_lt_fm_index) => {
                raw_lt_fm_index.lookup_table_kmer_size()
            },
        }
    }
    pub fn supported_utf8_letters(&self) ->  &[u8] {
        match self {
            Self::NO64(_) | Self::NO128(_) => b"ACGT",
            Self::NN64(_) | Self::NN128(_) => b"ACGT_",
            Self::AO64(_) | Self::AO128(_) => b"ACDEFGHIKLMNPQRSTVWY",
            Self::AN64(_) | Self::AN128(_) => b"ACDEFGHIKLMNPQRSTVWY_",
        }
    }
}
