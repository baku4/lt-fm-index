pub mod count_array;
pub mod bwt;

pub use count_array::CountArrayProto;
pub use crate::structure::CountArray;

pub use bwt::{BwtProto, BwtBlock};
pub use crate::structure::Bwt;