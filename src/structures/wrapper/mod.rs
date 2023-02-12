use crate::core::Text;
use super::{
    RawLtFmIndex,
    ChrIdxTable,
    BwtBlock,
};

pub struct LtFmIndex<E: TextEncoder> {
    inner: RawLtFmIndex<E::BwtBlockType>,
}
pub trait TextEncoder: Sized {
    type BwtBlockType: BwtBlock;

    fn encode(chrs_by_idx: &[&[u8]]) -> Self;
    fn chr_idx_table(&self) -> [u8; 256];
    fn chr_count(&self) -> usize;
}

// Build
impl<E: TextEncoder> LtFmIndex<E> {
    pub fn new(
        text: Text,
        text_encoder: &E,
        suffix_array_sampling_ratio: u64,
        lookup_table_kmer_size: u32,
    ) -> Self {
        let inner = RawLtFmIndex::new(
            text,
            suffix_array_sampling_ratio,
            lookup_table_kmer_size,
            ChrIdxTable(text_encoder.chr_idx_table()),
            text_encoder.chr_count(),
        );
        Self {
            inner
        }
    }
}
// Locate
impl<E: TextEncoder> LtFmIndex<E> {
    pub fn count(&self, pattern: &[u8]) -> u64 {
        self.inner.count(pattern)
    }
    pub fn locate(&self, pattern: &[u8]) -> Vec<u64> {
        self.inner.locate(pattern)
    }
}
