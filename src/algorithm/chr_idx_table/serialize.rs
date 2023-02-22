use crate::core::Serialize;
use super::ChrIdxTable;

impl Serialize for ChrIdxTable {
    fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write
    {
        writer.write_all(&self.0)?;
        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized
    {
        let mut buf = [0; 256];
        reader.read_exact(&mut buf)?;
        Ok(Self(buf))
    }
    fn to_be_saved_size(&self) -> usize {
        256
    }
}
