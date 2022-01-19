use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize, CheckBytes,
    Text, Pattern,
};
use super::{
    RawLtFmIndexShortPreBuild, RawLtFmIndexShort,
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

use nc_only::{TextEncoderNO, BwtBlock64NOPreBuild, BwtBlock64NO, BwtBlock128NOPreBuild, BwtBlock128NO};
use nc_with_noise::{TextEncoderNN, BwtBlock64NNPreBuild, BwtBlock64NN, BwtBlock128NNPreBuild, BwtBlock128NN};
use aa_only::{TextEncoderAO, BwtBlock64AOPreBuild, BwtBlock64AO, BwtBlock128AOPreBuild, BwtBlock128AO};
use aa_with_noise::{TextEncoderAN, BwtBlock64ANPreBuild, BwtBlock64AN, BwtBlock128ANPreBuild, BwtBlock128AN};

pub type LtFmIndexPreBuild64NO = RawLtFmIndexShortPreBuild<TextEncoderNO, BwtBlock64NOPreBuild>;
pub type LtFmIndexPreBuild128NO = RawLtFmIndexShortPreBuild<TextEncoderNO, BwtBlock128NOPreBuild>;
pub type LtFmIndexPreBuild64NN = RawLtFmIndexShortPreBuild<TextEncoderNN, BwtBlock64NNPreBuild>;
pub type LtFmIndexPreBuild128NN = RawLtFmIndexShortPreBuild<TextEncoderNN, BwtBlock128NNPreBuild>;
pub type LtFmIndexPreBuild64AO = RawLtFmIndexShortPreBuild<TextEncoderAO, BwtBlock64AOPreBuild>;
pub type LtFmIndexPreBuild128AO = RawLtFmIndexShortPreBuild<TextEncoderAO, BwtBlock128AOPreBuild>;
pub type LtFmIndexPreBuild64AN = RawLtFmIndexShortPreBuild<TextEncoderAN, BwtBlock64ANPreBuild>;
pub type LtFmIndexPreBuild128AN = RawLtFmIndexShortPreBuild<TextEncoderAN, BwtBlock128ANPreBuild>;

pub type LtFmIndex64NO = RawLtFmIndexShort<TextEncoderNO, BwtBlock64NOPreBuild>;
pub type LtFmIndex128NO = RawLtFmIndexShort<TextEncoderNO, BwtBlock128NOPreBuild>;
pub type LtFmIndex64NN = RawLtFmIndexShort<TextEncoderNN, BwtBlock64NNPreBuild>;
pub type LtFmIndex128NN = RawLtFmIndexShort<TextEncoderNN, BwtBlock128NNPreBuild>;
pub type LtFmIndex64AO = RawLtFmIndexShort<TextEncoderAO, BwtBlock64AOPreBuild>;
pub type LtFmIndex128AO = RawLtFmIndexShort<TextEncoderAO, BwtBlock128AOPreBuild>;
pub type LtFmIndex64AN = RawLtFmIndexShort<TextEncoderAN, BwtBlock64ANPreBuild>;
pub type LtFmIndex128AN = RawLtFmIndexShort<TextEncoderAN, BwtBlock128ANPreBuild>;
