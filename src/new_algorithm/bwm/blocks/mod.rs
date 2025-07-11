/*!
Implementation of blocks of trait [Block].
The suffix of the struct name is the count of inner vectors.
Using less vector makes the algorithms faster.
But the maximum count of the index is restrict to the $2^{v}-1$ ($v$ the vector count.)

[Block] uses [Vector] as inner vectors.
Currently, [Vector] is implemented for: u32, u64, u128.
The shorter the vector, the faster the algorithm, but the larger the struct.
*/
use super::Block;

mod vector;
mod block2;
mod block3;
mod block4;
mod block5;
mod block6;

pub use vector::Vector;
/// Block of 2 vectors that can index 3 characters.
pub use block2::Block2;
/// Block of 3 vectors that can index 7 characters.
pub use block3::Block3;
/// Block of 4 vectors that can index 15 characters.
pub use block4::Block4;
/// Block of 5 vectors that can index 31 characters.
pub use block5::Block5;
/// Block of 6 vectors that can index 63 characters.
pub use block6::Block6;