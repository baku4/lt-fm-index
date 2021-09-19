use std::fmt::Debug;

// Error propagation
use anyhow::Result;
use anyhow::bail as error_msg;
// Serialization
use serde::{Serialize, Deserialize};

mod structure;
mod proto;
mod use_case;
mod config;

pub mod deprecated;
mod tests;

pub type Text = Vec<u8>;
pub type Pattern<'a> = &'a [u8];

pub trait FmIndex {
    fn count(&self, pattern: Pattern) -> u64;
    fn locate(&self, pattern: Pattern) -> Vec<u64>;
}

pub use config::LtFmIndexConfig;