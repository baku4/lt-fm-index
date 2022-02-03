use std::{fs::File, io::{Read, Write}};

use super::FmIndexDep;

impl FmIndexDep {
    /// Write [FmIndex] to writer
    pub fn write_index_to<W>(&self, writer: W) -> Result<(), String>
        where W: Write 
    {
        match bincode::serialize_into(writer, self) {
            Ok(_) => Ok(()),
            Err(err) => {
                Err(format!("[bincode error] {}", err))
            },
        }
    }
    /// Write [FmIndex] to file
    pub fn write_index_to_file(&self, file_path: &str) -> Result<(), String> {
        let file = {
            match File::create(file_path) {
                Ok(file) => file,
                Err(err) => { return Err(format!("{}", err)); }
            }
        };
        self.write_index_to(file)
    }
    /// Read [FmIndex] from reader
    pub fn read_index_from<R>(reader: R) -> Result<Self, String>
        where R: Read 
    {
        match bincode::deserialize_from::<R, Self>(reader) {
            Ok(fm_index) => {
                Ok(fm_index)
            },
            Err(err) => {
                Err(format!("[bincode error]{:?}", err))
            },
        }
    }
    /// Read [FmIndex] from file
    pub fn read_index_from_file(file_path: &str) -> Result<Self, String> {
        let file = {
            match File::open(file_path) {
                Ok(file) => file,
                Err(err) => { return Err(format!("{}", err)); }
            }
        };
        Self::read_index_from(file)
    }
}
