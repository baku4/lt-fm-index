use crate::core::{
    Serializable,
};
use super::{
    LtFmIndex64NO, LtFmIndex128NO, LtFmIndex64NN, LtFmIndex128NN,
    LtFmIndex64AO, LtFmIndex128AO, LtFmIndex64AN, LtFmIndex128AN,
    LtFmIndexDep, InnerWrapper,
};
use thiserror::Error;

/// Errors that can occur when saving or loading LtFmIndex
#[derive(Error, Debug)]
pub enum IoError {
    /// std::io::Error
    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
    /// - This error can occur in various scenarios:
    ///   1. You pass Non-LtFmIndex data
    ///   2. You pass the LtFmIndex data of different OS
    ///     - the serialized bytes array of LtFmIndex can differ by pointer width or architecture.
    /// - This error sometimes can not be captured because the validation step is checking the "magic number" of the only first some bytes of data. When error is not occurred from invalid LtFmIndex data, thread **can be** panic or LtFmIndex structure does not works properly.
    #[error("not a valid LtFmIndex structure")]
    Invalid,
}

// TODO: Long magic number
const MAGIC_NUMBER_NO64: u8 = 11;
const MAGIC_NUMBER_NO128: u8 = 22;
const MAGIC_NUMBER_NN64: u8 = 33;
const MAGIC_NUMBER_NN128: u8 = 44;
const MAGIC_NUMBER_AO64: u8 = 55;
const MAGIC_NUMBER_AO128: u8 = 66;
const MAGIC_NUMBER_AN64: u8 = 77;
const MAGIC_NUMBER_AN128: u8 = 88;

impl LtFmIndexDep {
    pub fn save_to<W>(&self, mut writer: W) -> Result<(), IoError> where
        W: std::io::Write,
    {
        match &self.inner_wrapper {
            InnerWrapper::NO64(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_NO64])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            InnerWrapper::NO128(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_NO128])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            InnerWrapper::NN64(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_NN64])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            InnerWrapper::NN128(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_NN128])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            InnerWrapper::AO64(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_AO64])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            InnerWrapper::AO128(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_AO128])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            InnerWrapper::AN64(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_AN64])?;
                raw_lt_fm_index.save_to(writer)?;
            },
            InnerWrapper::AN128(raw_lt_fm_index) => {
                writer.write(&[MAGIC_NUMBER_AN128])?;
                raw_lt_fm_index.save_to(writer)?;
            },
        }

        Ok(())
    }
    pub fn load_from<R>(mut reader: R) -> Result<Self, IoError> where
        R: std::io::Read,
        Self: Sized,
    {
        let mut magic_number: [u8; 1] = [0; 1];
        reader.read_exact(&mut magic_number)?;

        let inner_wrapper = match magic_number[0] {
            MAGIC_NUMBER_NO64 => {
                InnerWrapper::NO64(LtFmIndex64NO::load_from(reader)?)
            },
            MAGIC_NUMBER_NO128 => {
                InnerWrapper::NO128(LtFmIndex128NO::load_from(reader)?)
            },
            MAGIC_NUMBER_NN64 => {
                InnerWrapper::NN64(LtFmIndex64NN::load_from(reader)?)
            },
            MAGIC_NUMBER_NN128 => {
                InnerWrapper::NN128(LtFmIndex128NN::load_from(reader)?)
            },
            MAGIC_NUMBER_AO64 => {
                InnerWrapper::AO64(LtFmIndex64AO::load_from(reader)?)
            },
            MAGIC_NUMBER_AO128 => {
                InnerWrapper::AO128(LtFmIndex128AO::load_from(reader)?)
            },
            MAGIC_NUMBER_AN64 => {
                InnerWrapper::AN64(LtFmIndex64AN::load_from(reader)?)
            },
            MAGIC_NUMBER_AN128 => {
                InnerWrapper::AN128(LtFmIndex128AN::load_from(reader)?)
            },
            _ => {
                return Err(IoError::Invalid)
            },
        };

        Ok(Self { inner_wrapper })
    }
    /// Bytes size of the file to be saved.
    pub fn size_of(&self) -> usize {
        1 // Magic number
        + match &self.inner_wrapper {
            InnerWrapper::NO64(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            InnerWrapper::NO128(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            InnerWrapper::NN64(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            InnerWrapper::NN128(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            InnerWrapper::AO64(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            InnerWrapper::AO128(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            InnerWrapper::AN64(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
            InnerWrapper::AN128(raw_lt_fm_index) => {
                raw_lt_fm_index.size_of()
            },
        }
    }
}