use crate::core::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
};

use crate::composition::{
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex
};

use std::marker::PhantomPinned;
use std::pin::Pin;
use rkyv::ser::{
    Serializer,
    serializers::AllocSerializer,
};

pub struct LtFmIndex {
    pinned_bytes: Pin<Box<Vec<u8>>>,
    casted_pointer: *const SelfDescLtFmIndex,
    _pinned: PhantomPinned,
}

impl LtFmIndex {
    pub fn count(&self, pattern: Pattern) -> u64 {
        unsafe{ self.casted_pointer.as_ref() }.unwrap().count(pattern)
    }
    pub fn locate(&self, pattern: Pattern) -> Vec<u64> {
        unsafe{ self.casted_pointer.as_ref() }.unwrap().locate(pattern)
    }
}
