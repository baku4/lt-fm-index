use super::{
    Result, error_msg,
    Text,
    EndianType, ReadBytesExt, WriteBytesExt, Serializable,
    SuffixArrayInterface,
};

#[allow(dead_code)]
mod burrow_wheeler_transform;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuffixArray {
    sampling_ratio: u64,
    array: Vec<u64>,
}

impl SuffixArrayInterface for SuffixArray {
    fn new_while_bwt(text: &mut Text, sa_sampling_ratio: u64) -> (Self, u64) {
        #[cfg(not(target_arch = "wasm32"))]
        let (suffix_array_i64, pidx) = burrow_wheeler_transform::get_suffix_array_and_pidx_while_bwt_not_for_wasm(text);
        #[cfg(target_arch = "wasm32")]
        let (suffix_array_i64, pidx) = burrow_wheeler_transform::get_suffix_array_and_pidx_while_bwt_for_wasm(text);

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
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write,
    {
        writer.write_u64::<EndianType>(self.sampling_ratio)?;

        self.array.save_to(writer)?;

        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self> where
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
}
