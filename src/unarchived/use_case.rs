/*! Implementation of [LtFmIndex] according to use cases.

In general, it is more recommended to use [crate::LtFmIndexConfig] instead of using [LtFmIndex] directly.  
Using the struct implemented in this module directly can occur `panic!`.

- **8** Case of LtFmIndex structure
  - By text type (4)
    - Nucleotide only
    - Nucleotide with noise
    - Aminoacid only
    - Aminoacid with noise
  - By bwt interval (2)
    - 64
    - 128 */

use super::{FmIndex, Text, Pattern};
use super::{Serialize, Deserialize};
use super::proto::{CountArray, CountArrayProto, BwtProto, BwtBlock};
pub use super::proto::LtFmIndex;

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

/// [LtFmIndex] of only nucleotide and 64 sized bwt block.
pub type LtFmIndexNO64 = LtFmIndex<CountArrayNO, BwtProto<BwtBlock64NO>>;
/// [LtFmIndex] of only nucleotide and 128 sized bwt block.
pub type LtFmIndexNO128 = LtFmIndex<CountArrayNO, BwtProto<BwtBlock128NO>>;
/// [LtFmIndex] of nucleotide with noise and 64 sized bwt block.
pub type LtFmIndexNN64 = LtFmIndex<CountArrayNN, BwtProto<BwtBlock64NN>>;
/// [LtFmIndex] of nucleotide with noise and 128 sized bwt block.
pub type LtFmIndexNN128 = LtFmIndex<CountArrayNN, BwtProto<BwtBlock128NN>>;
/// [LtFmIndex] of only amino acid and 64 sized bwt block.
pub type LtFmIndexAO64 = LtFmIndex<CountArrayAO, BwtProto<BwtBlock64AO>>;
/// [LtFmIndex] of only amino acid and 128 sized bwt block.
pub type LtFmIndexAO128 = LtFmIndex<CountArrayAO, BwtProto<BwtBlock128AO>>;
/// [LtFmIndex] of amino acid with noise and 64 sized bwt block.
pub type LtFmIndexAN64 = LtFmIndex<CountArrayAN, BwtProto<BwtBlock64AN>>;
/// [LtFmIndex] of amino acid with noise and 128 sized bwt block.
pub type LtFmIndexAN128 = LtFmIndex<CountArrayAN, BwtProto<BwtBlock128AN>>;

/// Wrapper for [LtFmIndex] to be generated safely from [crate::LtFmIndexConfig]
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum LtFmIndexAll {
    NO64(LtFmIndexNO64),
    NO128(LtFmIndexNO128),
    NN64(LtFmIndexNN64),
    NN128(LtFmIndexNN128),
    AO64(LtFmIndexAO64),
    AO128(LtFmIndexAO128),
    AN64(LtFmIndexAN64),
    AN128(LtFmIndexAN128),
}

impl FmIndex for LtFmIndexAll {
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