use std::marker::PhantomData;

use crate::core::{
    Text, Pattern,
    FmIndexInterface,
    LtFmIndexInterface,
};

mod suffix_array;
mod count_array;
mod bwm;



use suffix_array::SuffixArray;
use count_array::CountArray;

struct RawLtFmIndex<T: TextEncoder> {
    text_len: u64,
    chr_idx_table: ChrIdxTable,
    suffix_array: SuffixArray,
    count_array: CountArray,
    phantom_data: PhantomData<T>,
}
struct ChrIdxTable([u8; 256]);
impl ChrIdxTable {
    fn new(table: [u8; 256]) -> Self {
        Self(table)
    }
    fn idx_of(&self, chr: u8) -> u8 {
        unsafe {
            self.0.get_unchecked(chr)
        }
    }
}

pub trait TextEncoder {
    const CHR_COUNT: usize;

    // Build
    fn encode_chr_idx_table() -> [u8; 256];
    fn get_chridx_with_encoding_chr(unencoded_chr_utf8: &mut u8) -> usize;
    // Locate
    fn chrwpidx_of_chr(chr: u8) -> u32;
}

impl<T: TextEncoder> RawLtFmIndex<T> {
    // Build
    fn new(
        mut text: Text,
        suffix_array_sampling_ratio: u64,
        lookup_table_kmer_size: u32,
    ) -> Self {
        let text_len = text.len() as u64;
        let chr_idx_table = ChrIdxTable::new(T::encode_chr_idx_table());
        let chr_count = T::CHR_COUNT;
        let count_array = CountArray::<T>::new_and_encode_text(&mut text, &chr_idx_table, lookup_table_kmer_size);
        let (suffix_array, pidx) = SuffixArray::new_while_bwt(&mut text, suffix_array_sampling_ratio);
        // let bwt = B::new(text, pidx);
        // Self {
        //     text_len,
        //     suffix_array,
        //     count_array,
        //     bwt
        // }
        // FIXME:
        panic!("")
    }
    // Locate
    fn count(&self, pattern: Pattern) -> u64 {
        // FIXME:
        panic!("")
    }
    fn locate(&self, pattern: Pattern) -> Vec<u64> {
        // FIXME:
        panic!("")
    }
}
