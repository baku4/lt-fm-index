use super::{
    Result,
    Serializable,
};
use super::{
    SelfDescLtFmIndex,
};
use super::{
    LtFmIndex,
};


impl LtFmIndex {
    pub fn save_to<W>(&self, writer: W) -> Result<()> where
        W: std::io::Write,
    {
        self.self_desc_lt_fm_index.save_to(writer)?;

        Ok(())
    }
    pub fn load_from<R>(reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        let self_desc_lt_fm_index = SelfDescLtFmIndex::load_from(reader)?;

        Ok(Self {
            self_desc_lt_fm_index,
        })
    }
}
