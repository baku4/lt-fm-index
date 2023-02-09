use super::{
    LtFmIndex,
    TextEncoder,
    text_encoders,
};

mod configure;
mod build;

pub struct LtFmIndexBuilder {
    chrs_list: Vec<Vec<u8>>,
    bwt_block_size: BwtBlockSize,
    suffix_array_sampling_ratio: u64,
    lookup_table_kmer_size: Option<u32>,
}
pub enum BwtBlockSize {
    _64,
    _128,
}

use thiserror::Error;
/// Error thats can occur when build LtFmIndex with LtFmIndexBuilder
#[derive(Error, Debug)]
pub enum BuildError {
    /// [TextType] can not be inferred.
    /// This is occurred when the multiple characters must be assigned to wild-card(*).
    /// ex)
    ///  - ACG@@@@ -> Ok
    ///  - ACG@@## -> Error
    ///  - ACGT@@@@ -> Ok
    ///  - ACGT@@## -> Error
    #[error("The type of text can not be inferred ('{0}' and '{1}' cannot coexist).")]
    TextTypeError(char, char),
    #[error("Indexing the {0} types of character is not supported.")]
    UnsupportedChrCount(usize),
    /// Unsupported suffix array sampling ratio is inserted. (a value must be >= 1)
    #[error("Suffix array sampling ratio allows a value >= 1")]
    SasrBound,
    /// Unsupported lookup table kmer size is inserted. (a value must be >=2 and <= half the width of pointer)
    #[error("Lookup table kmer size allows a value >= 2 and <= half the width of pointer")]
    LtksBound,
}
