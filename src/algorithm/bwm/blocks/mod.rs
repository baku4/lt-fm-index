/*!
Implementation of blocks of trait [Block].
The suffix of the struct name is the count of inner vectors.
Using less vector makes the algorithms faster.
But the maximum count of the index is restrict to the $2^{v}-1$ ($v$ the vector count.)
*/
use super::Block;

mod vector;
mod block2;
mod block3;
mod block4;
mod block5;
mod block6;

pub use vector::Vector;
pub use block2::Block2;
pub use block3::Block3;
pub use block4::Block4;
pub use block5::Block5;
pub use block6::Block6;