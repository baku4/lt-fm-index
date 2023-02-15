use crate::{
    TextLen,
    Text,
    EndianType, ReadBytesExt, WriteBytesExt, Serialize,
};

#[allow(dead_code)]
mod burrow_wheeler_transform;
use burrow_wheeler_transform::get_compressed_suffix_array_and_pidx_while_bwt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuffixArray {
    sampling_ratio: TextLen,
    array: Vec<TextLen>,
}

impl SuffixArray {
    // Build
    pub fn new_while_bwt(text: &mut Text, sasr: TextLen) -> (Self, TextLen) {
        let (compressed_suffix_array, pidx) = get_compressed_suffix_array_and_pidx_while_bwt(text, sasr);

        let suffix_array = Self {
            sampling_ratio: sasr,
            array: compressed_suffix_array,
        };
        (suffix_array, pidx)
    }

    // Locate
    pub fn sampling_ratio(&self) -> TextLen {
        self.sampling_ratio
    }
    pub fn get_location_of(&self, position: TextLen) -> TextLen {
        self.array[(position / self.sampling_ratio) as usize]
    }
}

use capwriter::{Saveable, Loadable};

impl Serialize for SuffixArray {
    fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
        W: std::io::Write,
    {
        writer.write_u64::<EndianType>(self.sampling_ratio as u64)?;

        self.array.save_to(writer)?;

        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized,
    {
        let sampling_ratio = reader.read_u64::<EndianType>()? as TextLen;

        let array = Vec::<TextLen>::load_from(reader)?;

        Ok(Self{
            sampling_ratio,
            array,
        })
    }
    fn estimate_size(&self) -> usize {
        8 // sampling_ratio
        + self.array.size_of() // array
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compress_suffix_array(suffix_array: Vec<TextLen>, sampling_ratio: TextLen) -> Vec<TextLen> {
        if sampling_ratio == 1 {
            suffix_array.into_iter().map(|x| x as TextLen).collect()
        } else {
            suffix_array.into_iter().step_by(sampling_ratio as usize).map(|x| x ).collect()
        }
    }

    #[test]
    fn test_compress_suffix_array() {
        let raw_suffix_array: Vec<TextLen> = (0..30).collect();
        let sampling_ratio: TextLen = 5;
        let sa = compress_suffix_array(raw_suffix_array, sampling_ratio);
        assert_eq!(sa, vec![0, 5, 10, 15, 20, 25]);
    }
}
