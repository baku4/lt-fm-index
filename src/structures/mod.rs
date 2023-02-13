mod raw;
pub use raw::{
    RawLtFmIndex,
    ChrIdxTable,
    BwtBlock,
};

mod blocks;
pub use blocks::*;

mod wrapper;
pub use wrapper::{
    LtFmIndex,
    TextEncoder,
};

// Text encoders
// pub mod text_encoders;
