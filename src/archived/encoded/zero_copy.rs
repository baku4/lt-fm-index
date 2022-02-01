// Zero-copy version LtFmIndex
use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
    LtFmIndexConstructor, LtFmIndexInterface,
    SelfDescLtFmIndexPreBuild, SelfDescLtFmIndex, OptionPrint,
    TextType, BwtCompressionSize,
};

use rkyv::{
    check_archived_root, archived_root
};

pub struct CastedLtFmIndex<'a> {
    casted_pointer: &'a SelfDescLtFmIndex,
}

impl<'a> CastedLtFmIndex<'a> {
    #[inline]
    pub fn count(&self, pattern: Pattern) -> u64 {
        self.casted_pointer.count(pattern)
    }
    #[inline]
    pub fn locate(&self, pattern: Pattern) -> Vec<u64> {
        self.casted_pointer.locate(pattern)
    }
    pub fn new_from_bytes_checked(bytes: &'a [u8]) -> Result<Self> {
        let casted_pointer = match check_archived_root::<SelfDescLtFmIndexPreBuild>(bytes) {
            Ok(v) => {
                v
            },
            Err(_) => {
                error_msg!("Invalid lt-fm-index formatted bytes.")
            },
        };
        
        Ok(Self { casted_pointer })
    }
    pub fn new_from_bytes_unchecked(bytes: &'a [u8]) -> Self {
        let casted_pointer = unsafe { archived_root::<SelfDescLtFmIndexPreBuild>(bytes) };

        Self{ casted_pointer }
    }
}