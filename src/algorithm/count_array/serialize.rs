use crate::core::{Position, Serialize};
use super::CountArray;
use capwriter::{Save, Load};

impl<P: Position> Serialize for CountArray<P> {
    fn save_to<W>(&self, writer: &mut W) -> Result<(), std::io::Error> where
        W: std::io::Write,
    {
        // kmer_size
        self.kmer_size.save_as_ne(writer)?;

        // count_table
        self.count_table.save_as_ne(writer)?;

        // kmer_count_table
        self.kmer_count_table.save_as_ne(writer)?;

        // multiplier
        self.multiplier.save_as_ne(writer)?;

        Ok(())
    }
    fn load_from<R>(reader: &mut R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized,
    {
        // kmer_size
        let kmer_size = u32::load_as_ne(reader)?;

        // count_table
        let count_table = Vec::<P>::load_as_ne(reader)?;

        // kmer_count_table
        let kmer_count_table = Vec::<P>::load_as_ne(reader)?;

        // multiplier
        let multiplier = Vec::<usize>::load_as_ne(reader)?;

        Ok(Self {
            kmer_size,
            count_table,
            kmer_count_table,
            multiplier,
        })
    }
    fn encoded_len(&self) -> usize {
        4 // kmer_size
        + self.count_table.encoded_len() // count_table
        + self.kmer_count_table.encoded_len() // kmer_count_table
        + self.multiplier.encoded_len() // multiplier
    }
}
