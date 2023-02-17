use crate::core::{Position, Serialize, EndianType, WriteBytesExt, ReadBytesExt};
use super::{LtFmIndex, ChrIdxTable, SuffixArray, CountArray, Bwm, Block};

impl<P: Position, B: Block<P>> Serialize for LtFmIndex<P, B> {
    fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write
    {
        // text_len
        writer.write_u64::<EndianType>(self.text_len.as_u64())?;
        // chr_idx_table
        self.chr_idx_table.save_to(&mut writer)?;
        // suffix_array
        self.suffix_array.save_to(&mut writer)?;
        // count_array
        self.count_array.save_to(&mut writer)?;
        // bwm
        self.bwm.save_to(&mut writer)?;
        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized
    {
        let text_len = P::from_u64(reader.read_u64::<EndianType>()?);
        let chr_idx_table = ChrIdxTable::load_from(&mut reader)?;
        let suffix_array = SuffixArray::load_from(&mut reader)?;
        let count_array = CountArray::load_from(&mut reader)?;
        let bwm = Bwm::load_from(&mut reader)?;
        Ok(Self {
            text_len,
            chr_idx_table,
            suffix_array,
            count_array,
            bwm,
        })
    }
    fn to_be_saved_size(&self) -> usize {
        8 // text_len
        + self.chr_idx_table.to_be_saved_size() // chr_idx_table
        + self.suffix_array.to_be_saved_size() // suffix_array
        + self.count_array.to_be_saved_size() // count_array
        + self.bwm.to_be_saved_size() // bwm
    }
}
