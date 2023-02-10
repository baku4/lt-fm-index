use crate::core::Text;
use super::BwtBlock;
use bytemuck::{Pod, Zeroable};

// From 2 to 3 chrs
#[macro_use]
mod vec2;
// From 4 to 7 chrs
#[macro_use]
mod vec3;
// From 8 to 15 chrs
#[macro_use]
mod vec4;
// From 16 to 31 chrs
#[macro_use]
mod vec5;

macro_rules! Block {
    // Vec2
    ( $name: ident, 3, $bits: ty ) => {
        OuterBlock!($name, 3, 2, $bits);
        Vec2!($name, 3, $bits);
    };
    ( $name: ident, 4, $bits: ty ) => {
        OuterBlock!($name, 4, 2, $bits);
        Vec2!($name, 4, $bits);
    };
    // Vec3
    ( $name: ident, 5, $bits: ty ) => {
        OuterBlock!($name, 5, 3, $bits);
        Vec3!($name, 5, $bits);
    };
    ( $name: ident, 6, $bits: ty ) => {
        OuterBlock!($name, 6, 3, $bits);
        Vec3!($name, 6, $bits);
    };
}
macro_rules! OuterBlock {
    ( $name: ident, $chr: expr, $vec: expr, $bits: ty ) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
        pub struct $name {
            rank_checkpoint: [u64; $chr],
            bwt_vector: [$bits; $vec],
        }

        impl BwtBlock for $name {
            const BIT_LEN: u64 = <$bits>::BITS as u64;

            #[inline]
            fn new_with_bwt_text(bwt_text: Text) -> Vec<Self> {
                let mut chunk_count = bwt_text.len() / Self::BIT_LEN as usize;
                let rem = bwt_text.len() % Self::BIT_LEN as usize;
                
                let last_offset = if rem == 0 {
                    chunk_count += 1;
                    rem
                } else {
                    Self::BIT_LEN as usize - rem
                };
        
                let mut rank_checkpoint = [0; $chr];
                let mut blocks: Vec<Self> = Vec::with_capacity(chunk_count);
        
                bwt_text.chunks(Self::BIT_LEN as usize).for_each(|text_chunk| {
                    let block_rank_checkpoint = rank_checkpoint.clone();
                    let mut bwt_vector = [0; $vec];
                    Self::vectorize(text_chunk, &mut rank_checkpoint, &mut bwt_vector);
        
                    let block = Self {
                        rank_checkpoint: block_rank_checkpoint,
                        bwt_vector,
                    };
        
                    blocks.push(block);
                });
        
                if last_offset == 0 {
                    let last_block = Self {
                        rank_checkpoint,
                        bwt_vector: [0; $vec],
                    };
                    blocks.push(last_block);
                } else {
                    let last_block = blocks.last_mut().unwrap();
                    last_block.bwt_vector.iter_mut().for_each(|bits| *bits <<= last_offset);
                }
        
                blocks
            }
            #[inline]
            fn get_rank(&self, rem: u64, chridx: u8) -> u64 {
                self.get_rank_inner(rem, chridx)
            }
            #[inline]
            fn get_rank_and_chridx_of_rem(&self, rem: u64) -> (u64, u8) {
                self.get_rank_and_chridx_of_rem_inner(rem)
            }
        }
    };
}

// Implementations
Block!(V2U32, 3, u32);
Block!(V2U64, 3, u64);
Block!(V2U128, 3, u128);

Block!(V3U32, 4, u32);
Block!(V3U64, 4, u64);
Block!(V3U128, 4, u128);

Block!(V4U64, 5, u64);
Block!(V4U128, 5, u128);

Block!(V5U64, 6, u64);
Block!(V5U128, 6, u128);
