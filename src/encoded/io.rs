use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex,
    TextType, BwtCompressionSize,
    LtFmIndex,
};

use std::io::{Write, Read};
use std::path::Path;
use std::fs::File;
use std::pin::Pin;

impl LtFmIndex {
    pub fn save_to<W>(&self, mut writer: W) -> Result<usize> where W: Write {
        match writer.write(&self.bytes) {
            Ok(written_bytes) => Ok(written_bytes),
            Err(err) => error_msg!(err)
        }
    }
    pub fn save_to_file<P>(&self, file_path: P) -> Result<usize> where P: AsRef<Path> {
        let file = File::open(file_path)?;
        self.save_to(file)
    }
    pub fn load_from<R>(mut reader: R) -> Result<Self> where R: Read {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        Self::new_from_bytes_checked(buffer)
    }
    pub fn unchecked_load_from<R>(mut reader: R) -> Result<Self> where R: Read {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        Ok(Self::new_from_bytes_unchecked(buffer))
    }
    pub fn load_from_file<P>(file_path: P) -> Result<Self> where P: AsRef<Path> {
        let file = File::open(file_path)?;
        Self::load_from(file)
    }
}
