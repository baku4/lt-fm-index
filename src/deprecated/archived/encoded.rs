use super::core::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};
use super::composition::{
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex, OptionPrint,
    TextType, BwtCompressionSize,
};

use std::marker::PhantomPinned;
use std::pin::Pin;
use rkyv::{
    check_archived_root, archived_root
};

mod builder;
pub use builder::LtFmIndexBuilder;

mod zero_copy;
pub use zero_copy::CastedLtFmIndex;

// Additional features
mod raw;
mod io;
mod debug;
mod clone;

pub struct LtFmIndex {
    bytes: Pin<Box<Vec<u8>>>,
    casted_pointer: *const SelfDescLtFmIndex,
    _pinned: PhantomPinned,
}

impl LtFmIndex {
    #[inline]
    pub fn count(&self, pattern: Pattern) -> u64 {
        unsafe{ &*self.casted_pointer }.count(pattern)
    }
    #[inline]
    pub fn locate(&self, pattern: Pattern) -> Vec<u64> {
        unsafe{ &*self.casted_pointer }.locate(pattern)
    }
}

impl PartialEq for LtFmIndex {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for LtFmIndex {}
