// Text Length
mod text_length;
pub use text_length::Position;

// Serialize
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
    fn to_be_saved_size(&self) -> usize;
}

pub mod errors;