use crate::core::{
    Text, FmIndexInterface,
};
use super::{
    LtFmIndex,
    LtFmIndexBuilder,
    TextEncoder,
    BuildError,
    BwtBlockSize,
    text_encoders::*,
};

pub struct LtFmIndexRef {
    pointer: Box<dyn FmIndexInterface>,
}
impl<E: TextEncoder> FmIndexInterface for LtFmIndex<E> {
    fn count(&self, pattern: &[u8]) -> u64 {
        self.count(pattern)
    }
    fn locate(&self, pattern: &[u8]) -> Vec<u64> {
        self.locate(pattern)
    }
}

impl LtFmIndexBuilder {
    pub fn build(self, text: Text) -> Result<Box<dyn FmIndexInterface>, BuildError> {
        let chr_count = self.chrs_list.len();
        let ss = self.suffix_array_sampling_ratio;
        let lk = match self.lookup_table_kmer_size {
            Some(v) => v,
            None => recommend_kmer_size(chr_count),
        };

        let boxed: Box<dyn FmIndexInterface> = {
            match chr_count {
                3 => {
                    match self.bwt_block_size {
                        BwtBlockSize::_64 => {
                            let te = C3B64::from_vec_slices(&self.chrs_list).unwrap();
                            let lf_fm_index = LtFmIndex::new(text, &te, ss, lk);
                            Box::new(lf_fm_index)
                        },
                        BwtBlockSize::_128 => {
                            let te = C3B128::from_vec_slices(&self.chrs_list).unwrap();
                            let lf_fm_index = LtFmIndex::new(text, &te, ss, lk);
                            Box::new(lf_fm_index)
                        }
                    }
                },
                _ => return Err(BuildError::UnsupportedChrCount(chr_count))
            }
        };

        // Ok(LtFmIndexRef { pointer: boxed })
        Ok(boxed)
    }
}

/// This method recommend the size of kmer that makes the count table less than 2 MiB.  
/// The size of kmer count table is about (chr_count + 1)^kmer bytes.
pub fn recommend_kmer_size(chr_count: usize) -> u32 {
    if chr_count <= 2 { 13 }        // 1594323
    else if chr_count == 3 { 10 }   // 
    else if chr_count == 4 { 9 }
    else if chr_count == 5 { 8 }
    else if chr_count == 6 { 7 }
    else if chr_count <= 10 { 6 }
    else if chr_count <= 17 { 5 }
    else if chr_count <= 37 { 4 }
    else { 3 }
}
