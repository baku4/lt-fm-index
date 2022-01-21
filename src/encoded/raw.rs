use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};
use super::{
    SelfDescLtFmIndexPreBuild,
    LtFmIndex, LtFmIndexBuilder,
};

use std::marker::PhantomPinned;
use std::pin::Pin;
use super::{
    check_archived_root, archived_root
};

impl LtFmIndex {
    pub fn new_from_bytes_checked(bytes: Vec<u8>) -> Result<Self> {
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
    pub fn new_from_bytes_unchecked(bytes: Vec<u8>) -> Self {
        let pinned_boxed_bytes = Box::pin(bytes);

        let mut casted_pointer = std::ptr::null();
        casted_pointer = unsafe { archived_root::<SelfDescLtFmIndexPreBuild>(&pinned_boxed_bytes) };

        Self {
            bytes: pinned_boxed_bytes,
            casted_pointer: casted_pointer,
            _pinned: PhantomPinned,
        }
    }
    pub fn take_inner_bytes(self) -> Vec<u8> {
        *Pin::into_inner(self.bytes)
    }
}