pub mod count_array;
pub mod bwt;

pub use count_array::CountArrayProto;
pub use bwt::{BwtProto, BwtBlock};

pub use crate::structure::{LtFmIndex, CountArray, Bwt};
