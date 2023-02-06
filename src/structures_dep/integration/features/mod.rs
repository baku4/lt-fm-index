use super::{
    LtFmIndex64NO, LtFmIndex128NO, LtFmIndex64NN, LtFmIndex128NN,
    LtFmIndex64AO, LtFmIndex128AO, LtFmIndex64AN, LtFmIndex128AN,
    LtFmIndexDep, InnerWrapper, TextTypeDep, BwtBlockSizeDep,
};

mod debug;
mod io;
pub use io::IoError;