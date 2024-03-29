use crate::core::{Position, Serialize, EndianType, ReadBytesExt, WriteBytesExt};
use super::CountArray;
use capwriter::{Save, Load};

impl<P: Position> Serialize for CountArray<P> {
    fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write,
    {
        // kmer_size
        writer.write_u32::<EndianType>(self.kmer_size)?;

        // count_table
        self.count_table.save_to(&mut writer)?;

        // kmer_count_table
        self.kmer_count_table.save_to(&mut writer)?;

        // multiplier
        self.multiplier.save_to(&mut writer)?;

        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized,
    {
        // kmer_size
        let kmer_size = reader.read_u32::<EndianType>()?;

        // count_table
        let count_table = Vec::<P>::load_from(&mut reader)?;

        // kmer_count_table
        let kmer_count_table = Vec::<P>::load_from(&mut reader)?;

        // multiplier
        let multiplier = Vec::<usize>::load_from(&mut reader)?;

        Ok(Self {
            kmer_size,
            count_table,
            kmer_count_table,
            multiplier,
        })
    }
    fn to_be_saved_size(&self) -> usize {
        4 // kmer_size
        + self.count_table.to_be_saved_size() // count_table
        + self.kmer_count_table.to_be_saved_size() // kmer_count_table
        + self.multiplier.to_be_saved_size() // multiplier
    }
}
