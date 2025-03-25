use crate::core::{
    Position, Serialize, EndianType, WriteBytesExt, ReadBytesExt,
};
use super::{LtFmIndex, ChrIdxTable, SuffixArray, CountArray, Bwm, Block};

impl<P: Position, B: Block<P>> LtFmIndex<P, B> {
    pub fn save_to<W>(&self, writer: &mut W) -> Result<(), std::io::Error> where
        W: std::io::Write
    {
        // text_len
        writer.write_u64::<EndianType>(self.text_len.as_u64())?;
        // chr_idx_table
        self.chr_idx_table.save_to(writer)?;
        // suffix_array
        self.suffix_array.save_to(writer)?;
        // count_array
        self.count_array.save_to(writer)?;
        // bwm
        self.bwm.save_to(writer)?;
        Ok(())
    }
    pub fn load_from<R>(reader: &mut R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized
    {
        let text_len = P::from_u64(reader.read_u64::<EndianType>()?);
        let chr_idx_table = ChrIdxTable::load_from(reader)?;
        let suffix_array = SuffixArray::load_from(reader)?;
        let count_array = CountArray::load_from(reader)?;
        let bwm = Bwm::load_from(reader)?;
        Ok(Self {
            text_len,
            chr_idx_table,
            suffix_array,
            count_array,
            bwm,
        })
    }
    pub fn to_be_saved_size(&self) -> usize {
        8 // text_len
        + self.chr_idx_table.to_be_saved_size() // chr_idx_table
        + self.suffix_array.to_be_saved_size() // suffix_array
        + self.count_array.to_be_saved_size() // count_array
        + self.bwm.to_be_saved_size() // bwm
    }
}
