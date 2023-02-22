mod core;
pub use crate::core::{
    Position,
    errors,
};
mod algorithm;
pub use algorithm::{
    LtFmIndex,
    Block,
    blocks,
};

mod wrapper;

#[cfg(test)]
mod tests;
