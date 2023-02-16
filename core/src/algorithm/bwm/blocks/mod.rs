use crate::core::TextLength;
use super::Block;
use bytemuck::{Pod, Zeroable};

mod vector;
use vector::Vector;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TestBlockV2<V: Vector>([V; 2]);

impl<T: TextLength, V: Vector> Block<T> for TestBlockV2<V> {
    const BLOCK_LEN: u32 = V::BLOCK_LEN;

    #[inline]
    fn vectorize(text_chunk: &[u8], rank_pre_counts: &mut Vec<T>) -> Self {
        let mut bwt_vectors = [V::ZERO; 2];
        text_chunk.iter().for_each(|chridxwp| {
            let chridx = chridxwp - 1;
            rank_pre_counts[chridx as usize] += T::ONE;
            bwt_vectors[0] <<= V::ONE;
            if chridx & 0b01 != 0 {
                bwt_vectors[0] += V::ONE;
            }
            bwt_vectors[1] <<= V::ONE;
            if chridx & 0b10 != 0 {
                bwt_vectors[1] += V::ONE;
            }
        });
        Self(bwt_vectors)
    }
    fn empty() -> Self {
        Self([V::ZERO; 2])
    }
    fn shift_last_offset(&mut self, offset: u32) {
        self.0.iter_mut().for_each(|bits| *bits <<= offset);
    }
    #[inline]
    fn get_remain_count_of(&self, rem: u32, chridx: u8) -> u32 {
        let mut count_bits = match chridx {
            0 => !self.0[1] & !self.0[0], // 00
            1 => !self.0[1] & self.0[0],  // 01
            2 => self.0[1] & !self.0[0],  // 10
            _ => self.0[1] & self.0[0],   // 11
        };
        count_bits >>= V::BLOCK_LEN - rem;
        count_bits.count_ones()
    }
    #[inline]
    fn get_chridx_of(&self, rem: u32) -> u8 {
        let mov = V::BLOCK_LEN - rem - 1;
        let v1 = (self.0[0] >> V::from_u32(mov)).as_u8() & 1;
        let v2 = (self.0[1] >> V::from_u32(mov)).as_u8() & 1;
        v1 + (v2 << 1)
    }
}

unsafe impl<V: Vector> Zeroable for TestBlockV2<V> {}
unsafe impl<V: Vector + 'static> Pod for TestBlockV2<V> {}