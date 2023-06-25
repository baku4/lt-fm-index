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

#[cfg(test)]
mod tests;
