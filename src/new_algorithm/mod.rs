use crate::core::{
    Position,
    errors::BuildError,
};

// Components of FM-index
mod encoding_table;
use encoding_table::ChrEncodingTable;
mod count_array;
use count_array::{CountArrayHeader, CountArrayView, CountArrayBuildError};
mod suffix_array;
use suffix_array::{SuffixArrayHeader, SuffixArrayView};
mod bwm;
// use bwm::Bwm;
// pub use bwm::{Block, blocks};

/// FM-index using lookup table for first k-mer search.
/// This is a space-efficient implementation of the FM-index that uses a lookup table
/// for the first k-mer search to improve performance.
#[derive(Clone, PartialEq, Eq)]
pub struct  FmIndex<'a, P: Position> {
    // headers
    chr_encoding_table: ChrEncodingTable,
    count_array_header: CountArrayHeader,
    suffix_array_header: SuffixArrayHeader,
    // views
    count_array_view: CountArrayView<'a, P>,
    suffix_array_view: SuffixArrayView<'a, P>,
}

impl<'a,P: Position> FmIndex<'a, P> {
    pub fn build<T>(
        mut text: Vec<u8>,
        // Option
        suffix_array_sampling_ratio: u32,
        lookup_table_kmer_size: u32,
        characters_by_index: &[T],
    ) -> Self
    where
        T: AsRef<[u8]>,
    {
        if suffix_array_sampling_ratio == 0 {
            todo!("Write error message"); // FIXME: Write error message
        }
        if lookup_table_kmer_size == 0 {
            todo!("Write error message"); // FIXME: Write error message
        }

        // 1) Init
        let text_len = text.len() as u64;
        let chr_count = characters_by_index.len() as u32;
        // FIXME: Check if the chr_count is over the limit
        // if chr_count > B::MAX_CHR {
        //     todo!("Write error message"); // FIXME: Write error message
        // }
        //  1. Generate headers
        let chr_encoding_table = ChrEncodingTable::new(characters_by_index);
        let count_array_header = CountArrayHeader::new(chr_count, lookup_table_kmer_size);
        let suffix_array_header = SuffixArrayHeader::new(suffix_array_sampling_ratio, text_len);
        //  2. Calculate blob size


        // 2) Build Fm-Index

        todo!()
    }
}