use super::{
    Result,
    EndianType, ReadBytesExt, WriteBytesExt, Serializable,
    RawLtFmIndexShort,
    TextEncoder, BwtBlockInterface,
};

const POS_BIT_64: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
const POS_BIT_128: u128 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

// TODO: Deduplicate code
mod nc_only;
mod nc_with_noise;
mod aa_only;
mod aa_with_noise;

use nc_only::{TextEncoderNO, BwtBlock64NO, BwtBlock128NO};
use nc_with_noise::{TextEncoderNN, BwtBlock64NN, BwtBlock128NN};
use aa_only::{TextEncoderAO, BwtBlock64AO, BwtBlock128AO};
use aa_with_noise::{TextEncoderAN, BwtBlock64AN, BwtBlock128AN};

pub type LtFmIndex64NO = RawLtFmIndexShort<TextEncoderNO, BwtBlock64NO>;
pub type LtFmIndex128NO = RawLtFmIndexShort<TextEncoderNO, BwtBlock128NO>;
pub type LtFmIndex64NN = RawLtFmIndexShort<TextEncoderNN, BwtBlock64NN>;
pub type LtFmIndex128NN = RawLtFmIndexShort<TextEncoderNN, BwtBlock128NN>;
pub type LtFmIndex64AO = RawLtFmIndexShort<TextEncoderAO, BwtBlock64AO>;
pub type LtFmIndex128AO = RawLtFmIndexShort<TextEncoderAO, BwtBlock128AO>;
pub type LtFmIndex64AN = RawLtFmIndexShort<TextEncoderAN, BwtBlock64AN>;
pub type LtFmIndex128AN = RawLtFmIndexShort<TextEncoderAN, BwtBlock128AN>;
