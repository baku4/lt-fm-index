use super::{LtFmIndex, ChrIdxTable, SuffixArray, CountArray, Bwm, Block};

mod serialize;
#[cfg(feature = "async-tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "async-tokio")))]
mod async_serialize;
mod debug;
mod locate_from_reverse_raw_index;