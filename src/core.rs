// Text & Pattern
pub(crate) type Text = Vec<u8>;
pub(crate) type Pattern<'a> = &'a [u8];

// Text Length
#[cfg(not(features = "longtext"))]
pub type TextLen = u32;
#[cfg(features = "longtext")]
pub type TextLen = u64;

// Requirements
pub trait FmIndex {
    fn count(&self, pattern: Pattern) -> TextLen;
    fn locate(&self, pattern: Pattern) -> Vec<TextLen>;
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
