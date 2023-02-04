use super::{
    LtFmIndex,
    TextType,
    BwtBlockSize,
};

mod build;
mod configure;


#[derive(Debug, Clone)]
pub struct LtFmIndexBuilder {
    text_type: Option<TextType>,
    bwt_block_size: BwtBlockSize,
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
    #[error("Suffix array sampling ratio allows a value >= 1")]
    SasrBound,
    #[error("Lookup table kmer size allows a value >= 2 and <= half the width of pointer")]
    LtksBound,
}
