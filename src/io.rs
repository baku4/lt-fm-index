use super::{FmIndex};

use std::{fs::File, io::{Read, Write}};
/*
/// Write [FmIndex] to writer
pub fn write_index_to<W>(writer: W, fm_index: &dyn FmIndex) -> Result<(), String>
    where W: Write 
{
    match bincode::serialize_into(writer, fm_index) {
        Ok(_) => Ok(()),
        Err(err) => {
            Err(format!("[bincode error] {}", err))
        },
    }
}

/// Write [FmIndex] to file
pub fn write_index_to_file(file_path: &str, fm_index: &dyn FmIndex) -> Result<(), String> {
    let file = {
        match File::create(file_path) {
            Ok(file) => file,
            Err(err) => { return Err(format!("{}", err)); }
        }
    };
    write_index_to(file, fm_index)
}

/// Read [FmIndex] from reader
pub fn read_index_from<R>(reader: R) -> Result<Box<dyn FmIndex>, String>
    where R: Read 
{
    match bincode::deserialize_from::<R, dyn FmIndex>(reader) {
        Ok(fm_index) => {
            Ok(fm_index)
        },
        Err(err) => {
            Err(format!("[bincode error]{:?}", err))
        },
    }
}

/// Read [FmIndex] from file
pub fn read_index_from_file(file_path: &str) -> Result<Box<dyn FmIndex>, String> {
    let file = {
        match File::open(file_path) {
            Ok(file) => file,
            Err(err) => { return Err(format!("{}", err)); }
        }
    };
    read_index_from(file)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
    
    fn get_fmindex_toy() -> FmIndex {
        let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let config = FmIndexConfig::new()
            .set_kmer_lookup_table(7)
            .set_suffix_array_sampling_ratio(4);
        let fm_index = FmIndex::new(&config, text.clone());
        fm_index
    }

    #[test]
    fn index_write_and_read() {
        let mut buffer = Vec::new();
        // write
        let fm_index_1 = get_fmindex_toy();
        write_index_to(&mut buffer, &fm_index_1).unwrap();
        // read
        let fm_index_2 = read_index_from(&buffer[..]).unwrap();
        assert_eq!(fm_index_1, fm_index_2);
    }
}

 */