// Text & Pattern
pub(crate) type Text = Vec<u8>;
pub(crate) type Pattern<'a> = &'a [u8];

// Requirements
pub trait FmIndex {
    fn count(&self, pattern: Pattern) -> u64;
    fn locate(&self, pattern: Pattern) -> Vec<u64>;
}
pub trait Serialize {
    fn save_to<W>(&self, writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write;
    fn load_from<R>(reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized;
    fn estimate_size(&self) -> usize;
}

// For Serialization
#[cfg(target_endian = "little")]
pub(crate) type EndianType = byteorder::LittleEndian;
#[cfg(target_endian = "big")]
pub(crate) type EndianType = byteorder::BigEndian;
// Read & Write extension
pub use byteorder::{ReadBytesExt, WriteBytesExt};
