use crate::{FmIndex, Pattern};
use crate::{Serialize, Deserialize};
use crate::proto::{LtFmIndex, CountArray, CountArrayProto, BwtProto, BwtBlock};

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

#[derive(Debug, Serialize, Deserialize)]
pub enum LtFmIndexWrapper {
    NO64(LtFmIndexNO64),
    NO128(LtFmIndexNO128),
    NN64(LtFmIndexNN64),
    NN128(LtFmIndexNN128),
    AO64(LtFmIndexAO64),
    AO128(LtFmIndexAO128),
    AN64(LtFmIndexAN64),
    AN128(LtFmIndexAN128),
}

impl FmIndex for LtFmIndexWrapper {
    fn count(&self, pattern: Pattern) -> u64 {
        match self {
            Self::NO64(lt_fm_index) => lt_fm_index.count(pattern),
            Self::NO128(lt_fm_index) => lt_fm_index.count(pattern),
            Self::NN64(lt_fm_index) => lt_fm_index.count(pattern),
            Self::NN128(lt_fm_index) => lt_fm_index.count(pattern),
            Self::AO64(lt_fm_index) => lt_fm_index.count(pattern),
            Self::AO128(lt_fm_index) => lt_fm_index.count(pattern),
            Self::AN64(lt_fm_index) => lt_fm_index.count(pattern),
            Self::AN128(lt_fm_index) => lt_fm_index.count(pattern),
        }
    }
    fn locate(&self, pattern: Pattern) -> Vec<u64> {
        match self {
            Self::NO64(lt_fm_index) => lt_fm_index.locate(pattern),
            Self::NO128(lt_fm_index) => lt_fm_index.locate(pattern),
            Self::NN64(lt_fm_index) => lt_fm_index.locate(pattern),
            Self::NN128(lt_fm_index) => lt_fm_index.locate(pattern),
            Self::AO64(lt_fm_index) => lt_fm_index.locate(pattern),
            Self::AO128(lt_fm_index) => lt_fm_index.locate(pattern),
            Self::AN64(lt_fm_index) => lt_fm_index.locate(pattern),
            Self::AN128(lt_fm_index) => lt_fm_index.locate(pattern),
        }
    }
}