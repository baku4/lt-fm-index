use super::{Serialize, Deserialize};
use super::{Text, Pattern};
pub use super::structure::{LtFmIndex, CountArray, Bwt};

pub mod count_array;
pub mod bwt;

pub use count_array::CountArrayProto;
pub use bwt::{BwtProto, BwtBlock};
