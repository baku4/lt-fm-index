use crate::core::{
    Result, error_msg,
    Text, Pattern,
    LtFmIndexInterface, FmIndexInterface, Serializable,
};
use crate::composition::{
    SelfDescLtFmIndex,
    TextType, BwtCompressionSize,
};

// Builder
mod builder;
pub use builder::LtFmIndexBuilder;

// Features
mod feature;

#[derive(Clone, PartialEq, Eq)]
pub struct LtFmIndex {
    self_desc_lt_fm_index: SelfDescLtFmIndex,
}

impl LtFmIndex {
    fn new(self_desc_lt_fm_index: SelfDescLtFmIndex) -> Self {
        Self { self_desc_lt_fm_index }
    }

    pub fn count(&self, pattern: Pattern) -> u64 {
        self.self_desc_lt_fm_index.count(pattern)
    }
    pub fn locate(&self, pattern: Pattern) -> Vec<u64> {
        self.self_desc_lt_fm_index.locate(pattern)
    }
}
