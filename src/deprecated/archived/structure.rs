use super::core::{
    Result, error_msg,
    Archive, Serialize, Deserialize, CheckBytes,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};


// Foundation for basic structure

mod foundation;
use foundation::{
    // Type alias
    RawLtFmIndexShortPreBuild, RawLtFmIndexShort,
    // Requirements
    TextEncoder, BwtBlockConstructor, BwtBlockInterface,
};


// Construction of structure

mod construction;
pub use construction::{
    // LtFmIndexPreBuild by use case
    LtFmIndexPreBuild64NO, LtFmIndexPreBuild128NO, LtFmIndexPreBuild64NN, LtFmIndexPreBuild128NN,
    LtFmIndexPreBuild64AO, LtFmIndexPreBuild128AO, LtFmIndexPreBuild64AN, LtFmIndexPreBuild128AN,
    // LtFmIndex by use case
    LtFmIndex64NO, LtFmIndex128NO, LtFmIndex64NN, LtFmIndex128NN,
    LtFmIndex64AO, LtFmIndex128AO, LtFmIndex64AN, LtFmIndex128AN,
};
