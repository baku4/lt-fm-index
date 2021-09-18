use crate::{proto::{CountArray, CountArrayProto, BwtProto, BwtBlock}, structure::LtFmIndex};

const POS_BIT_64: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
const POS_BIT_128: u128 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;

mod nc_only;
mod nc_with_noise;
mod aa_only;
mod aa_with_noise;

use nc_only::{CountArrayNO, Bwt64NO, Bwt128NO};
use nc_with_noise::{CountArrayNN, Bwt64NN, Bwt128NN};
use aa_only::{CountArrayAO, Bwt64AO, Bwt128AO};
use aa_with_noise::{CountArrayAN, Bwt64AN, Bwt128AN};

pub type LtFmIndexNO64 = LtFmIndex<CountArrayNO, BwtProto<Bwt64NO>>;
pub type LtFmIndexNO128 = LtFmIndex<CountArrayNO, BwtProto<Bwt128NO>>;
pub type LtFmIndexNN64 = LtFmIndex<CountArrayNN, BwtProto<Bwt64NN>>;
pub type LtFmIndexNN128 = LtFmIndex<CountArrayNN, BwtProto<Bwt128NN>>;
pub type LtFmIndexAO64 = LtFmIndex<CountArrayAO, BwtProto<Bwt64AO>>;
pub type LtFmIndexAO128 = LtFmIndex<CountArrayAO, BwtProto<Bwt128AO>>;
pub type LtFmIndexAN64 = LtFmIndex<CountArrayAN, BwtProto<Bwt64AN>>;
pub type LtFmIndexAN128 = LtFmIndex<CountArrayAN, BwtProto<Bwt128AN>>;
