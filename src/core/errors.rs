use std::error::Error;
use std::fmt;

// #[derive(Debug)]
// pub enum LtFmIndexError {
//     BuildError(BuildError),
//     IoError(std::io::Error),
// }
// impl Error for LtFmIndexError {
//     fn source(&self) -> Option<&(dyn Error + 'static)> {
//         match *self {
//             Self::BuildError(_) => None,
//             Self::IoError(ref e) => Some(e),
//         }
//     }
// }
// impl fmt::Display for LtFmIndexError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match *self {
//             Self::BuildError(_) => {
//                 write!(f, "Error occurred in building process")
//             },
//             Self::IoError(_) => {
//                 write!(f, "Error occurred in serializing process")
//             },
//         }
//     }
// }
// impl From<BuildError> for LtFmIndexError {
//     fn from(err: BuildError) -> Self {
//         Self::BuildError(err)
//     }
// }

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
