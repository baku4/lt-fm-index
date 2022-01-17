pub use anyhow::{Result, bail as error_msg};
pub use rkyv::{Archive, Serialize, Deserialize};

// Text & Pattern
pub type Text = Vec<u8>;
pub type Pattern<'a> = &'a [u8];

// LtFmIndex Requirements
pub trait LtFmIndexConstructor {
    fn new(
        text: Text,
        suffix_array_sampling_ratio: u64,
        lookup_table_kmer_size: usize,
    ) -> Self;
}

pub trait LtFmIndexInterface {
    fn count(&self, pattern: Pattern) -> u64;
    fn locate(&self, pattern: Pattern) -> Vec<u64>;
}
