use crate::core::{Position, Serialize};
use super::SuffixArray;
use capwriter::{Save, Load};

impl<P: Position> Serialize for SuffixArray<P> {
    fn save_to<W>(&self, writer: &mut W) -> Result<(), std::io::Error> where
        W: std::io::Write,
    {
        self.sampling_ratio.as_u64().save_as_ne(writer)?;

        self.array.save_as_ne(writer)?;

        Ok(())
    }
    fn load_from<R>(reader: &mut R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized,
    {
        let sampling_ratio = P::from_u64(u64::load_as_ne(reader)?);

        let array = Vec::<P>::load_as_ne(reader)?;

        Ok(Self{
            sampling_ratio,
            array,
        })
    }
    fn encoded_len(&self) -> usize {
        8 // sampling_ratio
        + self.array.encoded_len() // array
    }
}
