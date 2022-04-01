use super::{
    Result, error_msg,
    Serializable,
};
use super::{
    LtFmIndex64NO, LtFmIndex128NO, LtFmIndex64NN, LtFmIndex128NN,
    LtFmIndex64AO, LtFmIndex128AO, LtFmIndex64AN, LtFmIndex128AN,
};
use super::{
    SelfDescLtFmIndex,
};

const MAGIC_NUMBER_NO64: u8 = 11;
const MAGIC_NUMBER_NO128: u8 = 22;
const MAGIC_NUMBER_NN64: u8 = 33;
const MAGIC_NUMBER_NN128: u8 = 44;
const MAGIC_NUMBER_AO64: u8 = 55;
const MAGIC_NUMBER_AO128: u8 = 66;
const MAGIC_NUMBER_AN64: u8 = 77;
const MAGIC_NUMBER_AN128: u8 = 88;

impl Serializable for SelfDescLtFmIndex {
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write,
    {
        match self {
            Self::NO64(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_NO64])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            Self::NO128(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_NO128])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            Self::NN64(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_NN64])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            Self::NN128(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_NN128])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            Self::AO64(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_AO64])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            Self::AO128(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_AO128])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            Self::AN64(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_AN64])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            Self::AN128(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_AN128])?;
                raw_lt_fm_index.save_to(writer)?;
            },
        }

        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        let mut magic_number: [u8; 1] = [0; 1];
        reader.read_exact(&mut magic_number)?;

        let self_desc_lt_fm_index = match magic_number[0] {
            MAGIC_NUMBER_NO64 => {
                Self::NO64(LtFmIndex64NO::load_from(reader)?)
            },
            MAGIC_NUMBER_NO128 => {
                Self::NO128(LtFmIndex128NO::load_from(reader)?)
            },
            MAGIC_NUMBER_NN64 => {
                Self::NN64(LtFmIndex64NN::load_from(reader)?)
            },
            MAGIC_NUMBER_NN128 => {
                Self::NN128(LtFmIndex128NN::load_from(reader)?)
            },
            MAGIC_NUMBER_AO64 => {
                Self::AO64(LtFmIndex64AO::load_from(reader)?)
            },
            MAGIC_NUMBER_AO128 => {
                Self::AO128(LtFmIndex128AO::load_from(reader)?)
            },
            MAGIC_NUMBER_AN64 => {
                Self::AN64(LtFmIndex64AN::load_from(reader)?)
            },
            MAGIC_NUMBER_AN128 => {
                Self::AN128(LtFmIndex128AN::load_from(reader)?)
            },
            _ => error_msg!("Invalid LtFmIndex")
        };

        Ok(self_desc_lt_fm_index)
    }
    fn size_of(&self) -> usize {
        1 // Magic number
        + match self {
            Self::NO64(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            Self::NO128(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            Self::NN64(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            Self::NN128(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            Self::AO64(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            Self::AO128(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            Self::AN64(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            Self::AN128(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
        }
    }
}