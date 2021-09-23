// Error propagation
use anyhow::Result;
use anyhow::bail as error_msg;
// Serialization
use serde::{Serialize, Deserialize, de::DeserializeOwned};

mod structure;
mod proto;
pub mod use_case;
pub mod io;
mod config;

#[doc(hidden)]
pub mod deprecated;
#[doc(hidden)]
pub mod tests;

pub type Text = Vec<u8>;
pub type Pattern<'a> = &'a [u8];

pub trait FmIndex {
    fn count(&self, pattern: Pattern) -> u64;
    fn locate(&self, pattern: Pattern) -> Vec<u64>;
}

pub use config::LtFmIndexConfig;