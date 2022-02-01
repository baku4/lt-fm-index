
// ## Main
// Core types and requirements for lt-fm-index
mod core;
// Data structure
mod structure;
// Integration of data structure
#[doc(hidden)] // Make public for benchmark, not assumed to be used by end-users.
pub mod composition;
// Encoded wrapper
mod encoded;

// # API
// Public Struct
pub use encoded::{LtFmIndex, LtFmIndexBuilder, CastedLtFmIndex};
// Public Enum
pub use composition::{TextType, BwtCompressionSize};
// Public Type
pub use self::core::{Text, Pattern};
