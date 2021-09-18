use crate::{proto::{CountArray, CountArrayProto, BwtProto, BwtBlock}, structure::LtFmIndex};

const POS_BIT_64: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
const POS_BIT_128: u128 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

mod nc_only;
mod nc_with_noise;
mod aa_only;
mod aa_with_noise;

use nc_only::{CountArrayNO, BwtBlock64NO, BwtBlock128NO};
use nc_with_noise::{CountArrayNN, BwtBlock64NN, BwtBlock128NN};
use aa_only::{CountArrayAO, BwtBlock64AO, BwtBlock128AO};
use aa_with_noise::{CountArrayAN, BwtBlock64AN, BwtBlock128AN};

pub type LtFmIndexNO64 = LtFmIndex<CountArrayNO, BwtProto<BwtBlock64NO>>;
pub type LtFmIndexNO128 = LtFmIndex<CountArrayNO, BwtProto<BwtBlock128NO>>;
pub type LtFmIndexNN64 = LtFmIndex<CountArrayNN, BwtProto<BwtBlock64NN>>;
pub type LtFmIndexNN128 = LtFmIndex<CountArrayNN, BwtProto<BwtBlock128NN>>;
pub type LtFmIndexAO64 = LtFmIndex<CountArrayAO, BwtProto<BwtBlock64AO>>;
pub type LtFmIndexAO128 = LtFmIndex<CountArrayAO, BwtProto<BwtBlock128AO>>;
pub type LtFmIndexAN64 = LtFmIndex<CountArrayAN, BwtProto<BwtBlock64AN>>;
pub type LtFmIndexAN128 = LtFmIndex<CountArrayAN, BwtProto<BwtBlock128AN>>;
