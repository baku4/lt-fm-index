use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BuildError {
    /// Index is over the maximum count of block
    IndexCountOver(u32, u32),
    /// Invalid lookup table k-mer size
    LookupTableKmerSize,
    /// Invalid suffix array sampling ratio
    SuffixArraySamplingRatio,
}
impl Error for BuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::IndexCountOver(max, input) => {
                write!(f, "Maximum index of block is {}, but input is {}.", max, input)
            },
            Self::LookupTableKmerSize => {
                write!(f, "Lookup table kmer size must be a positive integer.")
            },
            Self::SuffixArraySamplingRatio => {
                write!(f, "Suffix array sampling ratio must be a positive integer.")
            },
        }
    }
}
