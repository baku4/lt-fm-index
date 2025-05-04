use thiserror::Error;

/// Error type for building `LtFmIndex`.
#[derive(Debug, Error)]
pub enum BuildError {
    /// Index is over the maximum count of block
    #[error("Maximum index of block is {0}, but input is {1}.")]
    IndexCountOver(u32, u32),
    /// Invalid lookup table k-mer size
    #[error("Lookup table kmer size must be a positive integer")]
    LookupTableKmerSize,
    /// Invalid suffix array sampling ratio
    #[error("Suffix array sampling ratio must be a positive integer")]
    SuffixArraySamplingRatio,
}