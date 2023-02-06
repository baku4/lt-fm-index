// Common structures
mod commons;
use commons::{
    // Requirements
    TextEncoder,
    BwtBlockInterface,
    // Type alias
    RawLtFmIndexShort,
};

// Constructions for various types
mod construction;
use construction::{
    // use cases
    LtFmIndex64NO, LtFmIndex128NO, LtFmIndex64NN, LtFmIndex128NN,
    LtFmIndex64AO, LtFmIndex128AO, LtFmIndex64AN, LtFmIndex128AN,
};

// Integration of all use cases
mod integration;
pub use integration::{
    LtFmIndexDep,
    TextTypeDep,
    BwtBlockSizeDep,
    IoError,
};