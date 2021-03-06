use crate::core::{
    Result, error_msg,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
    EndianType, ReadBytesExt, WriteBytesExt, Serializable,
};


// Foundation for basic structure

mod foundation;
use foundation::{
    // Requirements
    TextEncoder, BwtBlockInterface,
    // Type alias
    RawLtFmIndexShort,
};


// Construction of structure

mod construction;
pub use construction::{
    // LtFmIndex by use case
    LtFmIndex64NO, LtFmIndex128NO, LtFmIndex64NN, LtFmIndex128NN,
    LtFmIndex64AO, LtFmIndex128AO, LtFmIndex64AN, LtFmIndex128AN,
};
