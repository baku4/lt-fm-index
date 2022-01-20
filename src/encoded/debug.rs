use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex, OptionPrint,
    TextType, BwtCompressionSize,
    LtFmIndex,
};

use std::fmt::{Debug, Formatter};

impl LtFmIndex {
    pub fn text_type(&self) -> TextType {
        unsafe{ &*self.casted_pointer }.text_type()
    }
    pub fn bwt_compression_size(&self) -> BwtCompressionSize {
        unsafe{ &*self.casted_pointer }.bwt_compression_size()
    }
    pub fn suffix_array_sampling_ratio(&self) -> u64 {
        unsafe{ &*self.casted_pointer }.suffix_array_sampling_ratio()
    }
    pub fn lookup_table_kmer_size(&self) -> usize {
        unsafe{ &*self.casted_pointer }.lookup_table_kmer_size()
    }
    pub fn supported_utf8_letters(&self) ->  &[u8] {
        unsafe{ &*self.casted_pointer }.supported_utf8_letters()
    }
}

impl Debug for LtFmIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LtFmIndex")
            .field("text_type", &self.text_type())
            .field("bwt_compression_size", &self.bwt_compression_size())
            .field("suffix_array_sampling_ratio", &self.suffix_array_sampling_ratio())
            .field("lookup_table_kmer_size", &self.lookup_table_kmer_size())
            .field("supported_utf8_letters", &self.supported_utf8_letters())
            .field("inner_byte_size", &self.bytes.len())
            .finish()
    }
}

use crate::LtFmIndexBuilder;
use crate::tests::random_text::*;
#[test]
fn test_print_new() {
    
    let text = rand_text_with_length(&UTF8_OF_NO, 10_000_000);

    let lt_fm_index = LtFmIndexBuilder::new().build(text);

    println!("{:#?}", lt_fm_index);
}