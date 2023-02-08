mod raw;
use raw::{
    RawLtFmIndex,
    ChrIdxTable,
    BwtBlock,
};

mod blocks;
use blocks::{
    Four64, Four128,
};

mod wrapper;
pub use wrapper::{
    FmIndex,
    TextEncoder,
};

// Text Encoders
// mod encoders;