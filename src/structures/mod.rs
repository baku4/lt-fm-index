mod raw;
use raw::{
    RawLtFmIndex,
    ChrIdxTable,
    BwtBlock,
};

mod blocks;
pub use blocks::{ // FIXME: un-public
    B4U64, B4U128,
    B3U128, B3U64,
};

mod wrapper;
pub use wrapper::{
    LtFmIndex,
    TextEncoder,
};

// Text Encoders
pub mod text_encoders;
