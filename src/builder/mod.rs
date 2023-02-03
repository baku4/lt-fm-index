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
#[derive(Error, Debug)]
pub enum BuildError {
    #[error("The type of text can not be inferred ('{0}' and '{1}' cannot coexist).")]
    TextTypeError(char, char),
    #[error("Suffix array sampling ratio allows a value >= 1")]
    SasrBound,
    #[error("Lookup table kmer size allows a value >= 2 and <= half the width of pointer")]
    LtksBound,
}
