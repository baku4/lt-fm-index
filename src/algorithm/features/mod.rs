use super::{LtFmIndex, ChrIdxTable, SuffixArray, CountArray, Bwm, Block};

mod serialize;
#[cfg(feature = "async-io")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-io")))]
mod async_serialize;
mod debug;
mod locate_from_reverse_raw_index;