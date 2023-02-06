// Text & Pattern
pub type Text = Vec<u8>;
pub type Pattern<'a> = &'a [u8];

// Requirements
pub trait LtFmIndexInterface: FmIndexInterface {
    fn new(
        text: Text,
        suffix_array_sampling_ratio: u64,
        lookup_table_kmer_size: usize,
    ) -> Self;
}
pub trait FmIndexInterface {
    fn count(&self, pattern: Pattern) -> u64;
    fn locate(&self, pattern: Pattern) -> Vec<u64>;
}
pub trait Serializable {
    fn save_to<W>(&self, writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write;
    fn load_from<R>(reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized;
    fn size_of(&self) -> usize;
}

// For Serialization
#[cfg(target_endian = "little")]
pub type EndianType = byteorder::LittleEndian;
#[cfg(target_endian = "big")]
pub type EndianType = byteorder::BigEndian;
// Read & Write extension
pub use byteorder::{ReadBytesExt, WriteBytesExt};
