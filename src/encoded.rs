use crate::core::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};
use crate::composition::{
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


// Additional features
mod io;
mod debug;

pub struct LtFmIndex {
    bytes: Pin<Box<Vec<u8>>>,
    casted_pointer: *const SelfDescLtFmIndex,
    _pinned: PhantomPinned,
}

impl LtFmIndex {
    pub fn count(&self, pattern: Pattern) -> u64 {
        unsafe{ &*self.casted_pointer }.count(pattern)
    }
    pub fn locate(&self, pattern: Pattern) -> Vec<u64> {
        unsafe{ &*self.casted_pointer }.locate(pattern)
    }
    fn checked_new_from_bytes(bytes: Vec<u8>) -> Result<Self> {
        let pinned_boxed_bytes = Box::pin(bytes);

        let mut casted_pointer = std::ptr::null();
        casted_pointer = match check_archived_root::<SelfDescLtFmIndexPreBuild>(&pinned_boxed_bytes) {
            Ok(v) => {
                v
            },
            Err(_) => {
                error_msg!("Invalid lt-fm-index formatted bytes.")
            },
        };
        
        Ok(Self {
            bytes: pinned_boxed_bytes,
            casted_pointer: casted_pointer,
            _pinned: PhantomPinned,
        })
    }
    fn unchecked_new_from_bytes(bytes: Vec<u8>) -> Self {
        let pinned_boxed_bytes = Box::pin(bytes);

        let mut casted_pointer = std::ptr::null();
        casted_pointer = unsafe { archived_root::<SelfDescLtFmIndexPreBuild>(&pinned_boxed_bytes) };

        Self {
            bytes: pinned_boxed_bytes,
            casted_pointer: casted_pointer,
            _pinned: PhantomPinned,
        }
    }
}

impl PartialEq for LtFmIndex {
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl Eq for LtFmIndex {}
