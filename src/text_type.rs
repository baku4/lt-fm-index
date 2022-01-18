use crate::core::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
};
use crate::structure::{
    LtFmIndexShortPreBuild,
    TextEncoder,
    BwtBlockConstructor, BwtBlockInterface,
};

const POS_BIT_64: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
const POS_BIT_128: u128 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

// TODO: Deduplicate code
mod nc_only;
mod nc_with_noise;
mod aa_only;
mod aa_with_noise;

pub use nc_only::{TextEncoderNO, BwtBlock64NOPreBuild, BwtBlock64NO, BwtBlock128NOPreBuild, BwtBlock128NO};
pub use nc_with_noise::{TextEncoderNN, BwtBlock64NNPreBuild, BwtBlock64NN, BwtBlock128NNPreBuild, BwtBlock128NN};
pub use aa_only::{TextEncoderAO, BwtBlock64AOPreBuild, BwtBlock64AO, BwtBlock128AOPreBuild, BwtBlock128AO};
pub use aa_with_noise::{TextEncoderAN, BwtBlock64ANPreBuild, BwtBlock64AN, BwtBlock128ANPreBuild, BwtBlock128AN};

pub type LtFmIndex64NO = LtFmIndexShortPreBuild<TextEncoderNO, BwtBlock64NOPreBuild>;
pub type LtFmIndex128NO = LtFmIndexShortPreBuild<TextEncoderNO, BwtBlock128NOPreBuild>;
pub type LtFmIndex64NN = LtFmIndexShortPreBuild<TextEncoderNN, BwtBlock64NNPreBuild>;
pub type LtFmIndex128NN = LtFmIndexShortPreBuild<TextEncoderNN, BwtBlock128NNPreBuild>;
pub type LtFmIndex64AO = LtFmIndexShortPreBuild<TextEncoderAO, BwtBlock64AOPreBuild>;
pub type LtFmIndex128AO = LtFmIndexShortPreBuild<TextEncoderAO, BwtBlock128AOPreBuild>;
pub type LtFmIndex64AN = LtFmIndexShortPreBuild<TextEncoderAN, BwtBlock64ANPreBuild>;
pub type LtFmIndex128AN = LtFmIndexShortPreBuild<TextEncoderAN, BwtBlock128ANPreBuild>;
