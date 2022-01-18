use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
};
use super::{
    BwtBlockConstructor, BwtBlockInterface,
};

use std::marker::PhantomData;

// BwtBlock Structure

// #[derive(Archive, Serialize, Deserialize)]
// #[archive(archived = "BwtBlock")]
// pub struct BwtBlockPreBuild<B, T, const R: usize, const V: usize, const N: usize> where
//     B: BwtBits,
//     T: TextVectorTable<N>,
// {
//     rank_checkpoint: [u64; R],
//     bwt_vector: [B; V],
//     text_vector_table: PhantomData<T>,
// }

use std::ops::{AddAssign, Shl, ShlAssign, Shr, ShrAssign};

trait BwtBits {
    const BIT_SIZE: u64;
    const POS_BIT: Self;

    fn zero() -> Self;
    fn one_bit_shift_left(&mut self);
    fn bits_shift_left(&mut self, bit: usize);
    fn bits_shift_right(&mut self, bit: u64);
}
