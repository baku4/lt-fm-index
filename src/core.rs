// Error propagation
pub use anyhow::{Result, bail as error_msg};

// # For Algorithm

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


// # For Serialization

// Endian type
#[cfg(target_endian = "little")]
pub type EndianType = byteorder::LittleEndian;
#[cfg(target_endian = "big")]
pub type EndianType = byteorder::BigEndian;

// Read & Write extension
pub use byteorder::{ReadBytesExt, WriteBytesExt};

// Trait
pub trait Serializable {
    fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write;
    fn load_from<R>(reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized;
    fn size_of(&self) -> usize;
}
