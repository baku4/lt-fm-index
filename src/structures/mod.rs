mod raw;
use raw::{
    RawLtFmIndex,
    ChrIdxTable,
    BwtBlock,
};

mod blocks;

mod wrapper;
pub use wrapper::{
    LtFmIndex,
    TextEncoder,
};

/// Text encoders
pub mod text_encoders;
