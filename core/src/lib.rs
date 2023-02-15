/*!
Library for core components of lt-fm-index
*/

// Text & Pattern
pub(crate) type Text = Vec<u8>;
pub(crate) type Pattern<'a> = &'a [u8];
// Text Length
#[cfg(not(features = "longtext"))]
pub(crate) type TextLen = u32;
#[cfg(features = "longtext")]
pub(crate) type TextLen = u64;
// Serialize data
#[cfg(target_endian = "little")]
pub(crate) type EndianType = byteorder::LittleEndian;
#[cfg(target_endian = "big")]
pub(crate) type EndianType = byteorder::BigEndian;
pub(crate) use byteorder::{ReadBytesExt, WriteBytesExt};
pub(crate) trait Serialize {
    fn save_to<W>(&self, writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write;
    fn load_from<R>(reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized;
    fn estimate_size(&self) -> usize;
}

mod algorithm;
pub use algorithm::{
    RawLtFmIndex,
    Block,
};
pub mod blocks;

#[cfg(test)]
mod tests;