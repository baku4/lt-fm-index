use crate::core::{Position, Serialize, EndianType, WriteBytesExt, ReadBytesExt};
use super::SuffixArray;
use capwriter::{Save, Load};

impl<P: Position> Serialize for SuffixArray<P> {
    fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write,
    {
        writer.write_u64::<EndianType>(self.sampling_ratio.as_u64())?;

        self.array.save_to(writer)?;

        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized,
    {
        let sampling_ratio = P::from_u64(reader.read_u64::<EndianType>()?);

        let array = Vec::<P>::load_from(reader)?;

        Ok(Self{
            sampling_ratio,
            array,
        })
    }
    fn to_be_saved_size(&self) -> usize {
        8 // sampling_ratio
        + self.array.to_be_saved_size() // array
    }
}
