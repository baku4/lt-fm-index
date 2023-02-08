use crate::core::Text;
use super::{BwtBlock, ChrIdxTable};
use bytemuck::{Pod, Zeroable};

#[macro_use]
mod vec2;

macro_rules! Block {
    ( $name: ident, 3, $bits: ty ) => {
        OuterBlock!($name, 3, 2, $bits);
        Vec2!($name, 3, $bits);
    };
    ( $name: ident, 4, $bits: ty ) => {
        OuterBlock!($name, 4, 2, $bits);
        Vec2!($name, 4, $bits);
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
Block!(B3U32, 3, u32);
Block!(B3U64, 3, u64);
Block!(B3U128, 3, u128);
Block!(B4U32, 4, u32);
Block!(B4U64, 4, u64);
Block!(B4U128, 4, u128);
