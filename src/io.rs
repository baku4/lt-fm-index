use crate::{Result, error_msg};
use crate::{DeserializeOwned, Serialize};

use crate::use_case::*;

use std::fs::File;
use std::io::{Read, Write};

trait IO: Serialize + DeserializeOwned {
    fn write_to_file(&self, file_path: &str) -> Result<()> {
        let file = {
            match File::create(file_path) {
                Ok(file) => file,
                Err(err) => error_msg!("{}", err),
            }
        };
        self.write_to(file)
    }
    fn read_from_file(file_path: &str) -> Result<Self> {
        let file = {
            match File::open(file_path) {
                Ok(file) => file,
                Err(err) => error_msg!("{}", err),
            }
        };
        Self::read_from(file)
    }
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

impl IO for LtFmIndexNO64  {}
impl IO for LtFmIndexNO128 {}
impl IO for LtFmIndexNN64  {}
impl IO for LtFmIndexNN128 {}
impl IO for LtFmIndexAO64  {}
impl IO for LtFmIndexAO128 {}
impl IO for LtFmIndexAN64  {}
impl IO for LtFmIndexAN128 {}
impl IO for LtFmIndexWrapper  {}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::*;
    use super::*;
    use crate::use_case::*;

    #[test]
    fn test_write_ltm() {
        let mut buffer = Vec::new();

        let text = b"ATTGGGGCGCGG".to_vec();
        let lt_fm_index = LtFmIndexNO64::new(text, 3, 4);

        bincode::serialize_into(&mut buffer, &lt_fm_index);

        println!("{:?}", buffer);

        let test: LtFmIndexNO64 = bincode::deserialize_from(&buffer[..]).unwrap();

        println!("{:?}", test);
    }
}