use super::{
    TextType, BwtCompressionSize,
    LtFmIndex,
};

use std::fmt::Debug;

impl LtFmIndex {
    pub fn text_type(&self) -> TextType {
        self.self_desc_lt_fm_index.text_type()
    }
    pub fn bwt_compression_size(&self) -> BwtCompressionSize {
        self.self_desc_lt_fm_index.bwt_compression_size()
    }
    pub fn suffix_array_sampling_ratio(&self) -> u64 {
        self.self_desc_lt_fm_index.suffix_array_sampling_ratio()
    }
    pub fn lookup_table_kmer_size(&self) -> usize {
        self.self_desc_lt_fm_index.lookup_table_kmer_size()
    }
    pub fn supported_utf8_letters(&self) ->  &[u8] {
        self.self_desc_lt_fm_index.supported_utf8_letters()
    }
}

impl Debug for LtFmIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LtFmIndex")
            .field("text_type", &self.text_type())
            .field("bwt_compression_size", &self.bwt_compression_size())
            .field("suffix_array_sampling_ratio", &self.suffix_array_sampling_ratio())
            .field("lookup_table_kmer_size", &self.lookup_table_kmer_size())
            .field("supported_utf8_letters", &self.supported_utf8_letters())
            .finish()
    }
}
