use super::{
    LtFmIndex,
    TextEncoder,
    LtFmIndexBuilder,
    BwtBlockSize,
    BuildError,
};

#[cfg(target_pointer_width = "32")]
const POINTER_WIDTH: u32 = 32;
#[cfg(target_pointer_width = "64")]
const POINTER_WIDTH: u32 = 64;

const DEFAULT_BBS: BwtBlockSize = BwtBlockSize::_128;
const DEFAULT_SASR: u64 = 2;
impl Default for LtFmIndexBuilder {
    fn default() -> Self {
        Self {
            chrs_list: Vec::new(),
            bwt_block_size: DEFAULT_BBS,
            suffix_array_sampling_ratio: DEFAULT_SASR,
            lookup_table_kmer_size: None,
        }
    }
}

impl LtFmIndexBuilder {
    /// New builder with default option
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add_chr(mut self, chr: &[u8]) -> Self {
        self.chrs_list.push(chr.to_vec());
        self
    }
    pub fn reset_chr(mut self) -> Self {
        self.chrs_list.clear();
        self
    }
    /// Use the default BWT block size
    pub fn bwt_block_size_is_default(mut self) -> Self {
        self.bwt_block_size = DEFAULT_BBS;
        self
    }
    /// Use the 64-sized BWT block
    pub fn bwt_block_size_is_64(mut self) -> Self {
        self.bwt_block_size = BwtBlockSize::_64;
        self
    }
    /// Use the 128-sized BWT block
    pub fn bwt_block_size_is_128(mut self) -> Self {
        self.bwt_block_size = BwtBlockSize::_128;
        self
    }
    /// Use the default suffix array sampling ratio
    pub fn set_suffix_array_sampling_ratio_to_default(mut self) -> Self {
        self.suffix_array_sampling_ratio = DEFAULT_SASR;
        self
    }
    /// Set the suffix array sampling ratio
    pub fn set_suffix_array_sampling_ratio(
        mut self,
        sampling_ratio: u64,
    ) -> Result<Self, BuildError> {
        if sampling_ratio > 0 {
            self.suffix_array_sampling_ratio = sampling_ratio;
            Ok(self)
        } else {
            Err(BuildError::SasrBound)
        }
    }
    /// Use the default kmer size for lookup table
    pub fn set_lookup_table_kmer_size_to_default(mut self) -> Self {
        self.lookup_table_kmer_size = None;
        self
    }
    /// Set the size of kmer in lookup table
    pub fn set_lookup_table_kmer_size(
        mut self,
        kmer_size: u32,
    ) -> Result<Self, BuildError> {
        let max_kmer = POINTER_WIDTH / 2;
        
        if kmer_size < 2 || kmer_size > max_kmer {
            Err(BuildError::LtksBound)
        } else {
            self.lookup_table_kmer_size = Some(kmer_size);
            Ok(self)
        }
    }
}
