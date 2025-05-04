use crate::core::{Position, Serialize};
use super::{LtFmIndex, ChrIdxTable, SuffixArray, CountArray, Bwm, Block};
use capwriter::{Save, Load};

impl<P: Position, B: Block<P>> LtFmIndex<P, B> {
    pub fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write
    {
        // text_len
        self.text_len.as_u64().save_as_ne(&mut writer)?;
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
    pub fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized
    {
        let text_len = P::from_u64(u64::load_as_ne(&mut reader)?);
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
    pub fn encoded_len(&self) -> usize {
        8 // text_len
        + self.chr_idx_table.encoded_len() // chr_idx_table
        + self.suffix_array.encoded_len() // suffix_array
        + self.count_array.encoded_len() // count_array
        + self.bwm.encoded_len() // bwm
    }
}
