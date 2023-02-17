use crate::core::{Position};
use super::{LtFmIndex, Block};
use std::fmt::Debug;

impl<P: Position, B: Block<P>> Debug for LtFmIndex<P, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LtFmIndex")
            .field("bit_size_for_position", &P::BITS)
            .field("text_length", &self.len_text())
            .field("index_count", &self.index_count())
            .field("lookup_table_kmer_size", &self.lookup_table_kmer_size())
            .field("suffix_array_sampling_ratio", &self.suffix_array_sampling_ratio())
            .finish()
    }
}

impl<P: Position, B: Block<P>> LtFmIndex<P, B> {
    pub fn len_text(&self) -> P {
        self.text_len
    }
    pub fn index_count(&self) -> u32 {
        self.bwm.chr_count()
    }
    pub fn lookup_table_kmer_size(&self) -> u32 {
        self.count_array.kmer_size()
    }
    pub fn suffix_array_sampling_ratio(&self) -> P {
        self.suffix_array.sampling_ratio()
    }
}
