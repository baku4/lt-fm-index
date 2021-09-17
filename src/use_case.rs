use crate::proto::{CountArray, CountArrayProto, BwtProto, BwtBlock};

const POS_BIT_64: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
const POS_BIT_128: u128 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

mod only_nc;
mod nc_with_noise;
mod only_aa;
mod aa_with_noise;

pub use only_nc::{CountArrayON, Bwt64ON, Bwt128ON};
pub use nc_with_noise::{CountArrayNN, Bwt64NN, Bwt128NN};
pub use only_aa::{CountArrayOA, Bwt64OA, Bwt128OA};
pub use aa_with_noise::{CountArrayAN, Bwt64AN, Bwt128AN};