// Error propagation
use anyhow::Result;
use anyhow::bail as error_msg;
// Serialization
use serde::{Serialize, Deserialize, de::DeserializeOwned};

#[doc(hidden)]
#[cfg(not(target_arch = "wasm32"))]
#[allow(dead_code)]
pub mod deprecated;

pub mod structure;
pub mod proto;
pub mod use_case;
mod config;
mod io;

/// [Text] to generate [LtFmIndex]
pub type Text = Vec<u8>;
/// [Pattern] to use in [LtFmIndex]
pub type Pattern<'a> = &'a [u8];

/// [FmIndex] can count and locate [Pattern].
pub trait FmIndex {
    /// Count the number of times the [Pattern] appears.
    fn count(&self, pattern: Pattern) -> u64;
    /// Locate the start index in which the [Pattern] appears. The result vector is not sorted.
    fn locate(&self, pattern: Pattern) -> Vec<u64>;
}

pub use use_case::LtFmIndexAll;
pub use config::LtFmIndexConfig;
pub use io::IO;
