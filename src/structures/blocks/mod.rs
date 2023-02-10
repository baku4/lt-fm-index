use crate::core::Text;
use super::BwtBlock;
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
pub struct V2U64([u64; 2]);

impl BwtBlock for V2U64 {
    const BLOCK_LEN: u64 = 64;

    #[inline]
    fn vectorize(text_chunk: &[u8], rank_pre_counts: &mut Vec<u64>) -> Self {
        let mut bwt_vectors = [0; 2];
        text_chunk.iter().for_each(|chridxwp| {
            let chridx = chridxwp - 1;
            rank_pre_counts[chridx as usize] += 1;
            bwt_vectors[0] <<= 1;
            if chridx & 0b01 != 0 {
                bwt_vectors[0] += 1;
            }
            bwt_vectors[1] <<= 1;
            if chridx & 0b10 != 0 {
                bwt_vectors[1] += 1;
            }
        });
        Self(bwt_vectors)
    }
    #[inline]
    fn get_remain_count_of(&self, rem: u64, chridx: u8) -> u64 {
        let mut count_bits = match chridx {
            0 => !self.0[1] & !self.0[0], // 00
            1 => !self.0[1] & self.0[0],  // 01
            2 => self.0[1] & !self.0[0],  // 10
            _ => self.0[1] & self.0[0],   // 11
        };
        count_bits >>= (Self::BLOCK_LEN - rem) as u64;
        count_bits.count_ones() as _
    }
    #[inline]
    fn get_chridx_of(&self, rem: u64) -> u8 {
        let mov = Self::BLOCK_LEN - rem - 1;
        let v1 = (self.0[0] >> mov as u64) as u8 & 1;
        let v2 = (self.0[1] >> mov as u64) as u8 & 1 ;
        v1 + (v2 << 1)
    }
}


macro_rules! OuterBlock {
    ( $name: ident, $vec: expr, $bits: ty ) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
        pub struct $name([$bits; $vec])

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
