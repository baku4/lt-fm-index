use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex, OptionPrint,
    TextType, BwtCompressionSize,
    LtFmIndex,
};

use std::{clone::Clone, marker::PhantomPinned};
use super::archived_root;

impl Clone for LtFmIndex {
    fn clone(&self) -> Self {
        let cloned_bytes = self.bytes.clone();

        let mut casted_pointer = std::ptr::null();
        casted_pointer = unsafe { archived_root::<SelfDescLtFmIndexPreBuild>(&cloned_bytes) };

        Self {
            bytes: cloned_bytes,
            casted_pointer: casted_pointer,
            _pinned: PhantomPinned,
        }
    }
}
