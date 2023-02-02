use crate::core::{
    Text,
    EndianType, ReadBytesExt, WriteBytesExt, Serializable,
};
use super::SuffixArrayInterface;

mod burrow_wheeler_transform;
use burrow_wheeler_transform::get_suffix_array_and_pidx_while_bwt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuffixArray {
    sampling_ratio: u64,
    array: Vec<u64>,
}

impl SuffixArrayInterface for SuffixArray {
    fn new_while_bwt(text: &mut Text, sa_sampling_ratio: u64) -> (Self, u64) {
        let (suffix_array_i64, pidx) = get_suffix_array_and_pidx_while_bwt(text);

        let compressed_array = Self::compress_suffix_array(suffix_array_i64, sa_sampling_ratio);

        let suffix_array = Self {
            sampling_ratio: sa_sampling_ratio,
            array: compressed_array,
        };
        (suffix_array, pidx)
    }

    fn sampling_ratio(&self) -> u64 {
        self.sampling_ratio
    }
    fn get_location_of_position(&self, position: u64) -> u64 {
        self.array[(position / self.sampling_ratio) as usize]
    }
}

impl SuffixArray {
    fn compress_suffix_array(suffix_array: Vec<i64>, sampling_ratio: u64) -> Vec<u64> {
        if sampling_ratio == 1 {
            suffix_array.into_iter().map(|x| x as u64).collect()
        } else {
            suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x as u64).collect()
        }
    }
}

use capwriter::{Saveable, Loadable};

impl Serializable for SuffixArray {
    fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write,
    {
        writer.write_u64::<EndianType>(self.sampling_ratio)?;

        self.array.save_to(writer)?;

        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized,
    {
        let sampling_ratio = reader.read_u64::<EndianType>()?;

        let array = Vec::<u64>::load_from(reader)?;

        Ok(Self{
            sampling_ratio,
            array,
        })
    }
    fn size_of(&self) -> usize {
        8 // sampling_ratio
        + self.array.size_of() // array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_suffix_array() {
        let raw_suffix_array: Vec<i64> = (0..30).collect();
        let sampling_ratio: u64 = 5;
        let sa = SuffixArray::compress_suffix_array(raw_suffix_array, sampling_ratio);
        assert_eq!(sa, vec![0, 5, 10, 15, 20, 25]);
    }
}
