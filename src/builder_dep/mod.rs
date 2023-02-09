use super::{
    LtFmIndexDep,
    TextTypeDep,
    BwtBlockSizeDep,
};

mod build;
mod configure;

/// The safe and concise builder for LtFmIndex
#[derive(Debug, Clone)]
pub struct LtFmIndexBuilderDep {
    text_type: Option<TextTypeDep>,
    bwt_block_size: BwtBlockSizeDep,
    suffix_array_sampling_ratio: u64,
    lookup_table_kmer_size: Option<usize>,
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
    /// Unsupported suffix array sampling ratio is inserted. (a value must be >= 1)
    #[error("Suffix array sampling ratio allows a value >= 1")]
    SasrBound,
    /// Unsupported lookup table kmer size is inserted. (a value must be >=2 and <= half the width of pointer)
    #[error("Lookup table kmer size allows a value >= 2 and <= half the width of pointer")]
    LtksBound,
}
