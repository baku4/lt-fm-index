use super::{Result, error_msg};
use super::{DeserializeOwned, Serialize};

use super::use_case::*;

use std::fs::File;
use std::io::{Read, Write};

/// Trait for write and read
///
/// All [LtFmIndex] in [crate::use_case] have this trait
pub trait IO: Serialize + DeserializeOwned {
    /// Write to file
    fn write_to_file(&self, file_path: &str) -> Result<()> {
        let file = {
            match File::create(file_path) {
                Ok(file) => file,
                Err(err) => error_msg!("{}", err),
            }
        };
        self.write_to(file)
    }
    /// Read from file
    fn read_from_file(file_path: &str) -> Result<Self> {
        let file = {
            match File::open(file_path) {
                Ok(file) => file,
                Err(err) => error_msg!("{}", err),
            }
        };
        Self::read_from(file)
    }
    /// Write to [Write]r
    fn write_to<W>(&self, writer: W) -> Result<()>
        where W: Write 
    {
        match bincode::serialize_into(writer, self) {
            Ok(_) => Ok(()),
            Err(err) => {
                error_msg!("{}", err)
            },
        }
    }
    /// Read from [Read]r
    fn read_from<R>(reader: R) -> Result<Self>
        where R: Read,
    {
        match bincode::deserialize_from::<R, Self>(reader) {
            Ok(v) => {
                Ok(v)
            },
            Err(err) => {
                error_msg!("{}", err)
            },
        }
    }
}

impl IO for LtFmIndexNO64 {}
impl IO for LtFmIndexNO128 {}
impl IO for LtFmIndexNN64 {}
impl IO for LtFmIndexNN128 {}
impl IO for LtFmIndexAO64 {}
impl IO for LtFmIndexAO128 {}
impl IO for LtFmIndexAN64 {}
impl IO for LtFmIndexAN128 {}
impl IO for LtFmIndexAll {}
