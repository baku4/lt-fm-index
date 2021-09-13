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

#[cfg(test)]
mod tests {
    use crate::deprecated::*;

    fn get_fmindex_on() -> FmIndexDep {
        let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let config = FmIndexConfigDep::new()
            .set_kmer_lookup_table(7)
            .set_suffix_array_sampling_ratio(4);
        let fm_index = config.generate_fmindex(text.clone());
        fm_index
    }
    
    fn get_fmindex_nn() -> FmIndexDep {
        let text = "CTCCGTACACCTGTTTCGTATCGGAACCGGTAAGTGAAATTTCCACATCGCCGGAAACCGTATATTGTCCATCCGCTGCCGGTGGATCCGGCTCCTGCGTGGAAAACCAGTCATCCTGATTTACATATGGTTCAATGGCACCGGATGCATAGATTTCCCCATTTTGCGTACCGGAAACGTGCGCAAGCACGATCTGTGTCTTACC".as_bytes().to_vec();
        let config = FmIndexConfigDep::new()
            .set_kmer_lookup_table(7)
            .set_suffix_array_sampling_ratio(4)
            .contain_non_nucleotide();
        let fm_index = config.generate_fmindex(text.clone());
        fm_index
    }

    #[test]
    fn test_fmindex_on_write_and_read() {
        let mut buffer = Vec::new();
        // write
        let fm_index_on_to_write = get_fmindex_on();
        fm_index_on_to_write.write_index_to(&mut buffer).unwrap();
        // read
        let fm_index_on_readed = FmIndexDep::read_index_from(&buffer[..]).unwrap();
        assert_eq!(fm_index_on_to_write, fm_index_on_readed);
    }
    #[test]
    fn test_fmindex_nn_write_and_read() {
        let mut buffer = Vec::new();
        // write
        let fm_index_nn_to_write = get_fmindex_nn();
        fm_index_nn_to_write.write_index_to(&mut buffer).unwrap();
        // read
        let fm_index_nn_readed = FmIndexDep::read_index_from(&buffer[..]).unwrap();
        assert_eq!(fm_index_nn_to_write, fm_index_nn_readed);
    }
}
