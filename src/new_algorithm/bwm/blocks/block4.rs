use crate::core::Position;
use super::{Block, Vector};

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Block4<V: Vector>([V; 4]);

impl<V: Vector> Block for Block4<V> {
    const BLOCK_LEN: u32 = V::BLOCK_LEN;
    const MAX_CHR: u32 = 15;

    #[inline]
    fn vectorize<P: Position>(text_chunk: &[u8], rank_pre_counts: &mut Vec<P>) -> Self {
        let mut bwt_vectors = [V::ZERO; 4];
        text_chunk.iter().for_each(|chridxwp| {
            let chridx = chridxwp - 1;
            rank_pre_counts[chridx as usize] += P::ONE;
            bwt_vectors[0] <<= V::ONE;
            if chridx & 0b0001 != 0 {
                bwt_vectors[0] += V::ONE;
            }
            bwt_vectors[1] <<= V::ONE;
            if chridx & 0b0010 != 0 {
                bwt_vectors[1] += V::ONE;
            }
            bwt_vectors[2] <<= V::ONE;
            if chridx & 0b0100 != 0 {
                bwt_vectors[2] += V::ONE;
            }
            bwt_vectors[3] <<= V::ONE;
            if chridx & 0b1000 != 0 {
                bwt_vectors[3] += V::ONE;
            }
        });
        Self(bwt_vectors)
    }
    fn shift_last_offset(&mut self, offset: u32) {
        self.0.iter_mut().for_each(|bits| *bits <<= offset);
    }
    #[inline]
    fn get_remain_count_of(&self, rem: u32, chridx: u8) -> u32 {
        let mut count_bits = match chridx {
            0 => !self.0[3] & !self.0[2] & !self.0[1] & !self.0[0], // 0000
            1 => !self.0[3] & !self.0[2] & !self.0[1] & self.0[0],  // 0001
            2 => !self.0[3] & !self.0[2] & self.0[1] & !self.0[0],  // 0010
            3 => !self.0[3] & !self.0[2] & self.0[1] & self.0[0],   // 0011
            4 => !self.0[3] & self.0[2] & !self.0[1] & !self.0[0],  // 0100
            5 => !self.0[3] & self.0[2] & !self.0[1] & self.0[0],   // 0101
            6 => !self.0[3] & self.0[2] & self.0[1] & !self.0[0],   // 0110
            7 => !self.0[3] & self.0[2] & self.0[1] & self.0[0],    // 0111
            8 => self.0[3] & !self.0[2] & !self.0[1] & !self.0[0],  // 1000
            9 => self.0[3] & !self.0[2] & !self.0[1] & self.0[0],   // 1001
            10 => self.0[3] & !self.0[2] & self.0[1] & !self.0[0],  // 1010
            11 => self.0[3] & !self.0[2] & self.0[1] & self.0[0],   // 1011
            12 => self.0[3] & self.0[2] & !self.0[1] & !self.0[0],  // 1100
            13 => self.0[3] & self.0[2] & !self.0[1] & self.0[0],   // 1101
            14 => self.0[3] & self.0[2] & self.0[1] & !self.0[0],   // 1110
            _ => self.0[3] & self.0[2] & self.0[1] & self.0[0],     // 1111
        };
        count_bits >>= V::BLOCK_LEN - rem;
        count_bits.count_ones()
    }
    #[inline]
    fn get_chridx_of(&self, rem: u32) -> u8 {
        let mov = V::BLOCK_LEN - rem - 1;
        let v1 = (self.0[0] >> V::from_u32(mov)).as_u8() & 1;
        let v2 = (self.0[1] >> V::from_u32(mov)).as_u8() & 1;
        let v3 = (self.0[2] >> V::from_u32(mov)).as_u8() & 1;
        let v4 = (self.0[3] >> V::from_u32(mov)).as_u8() & 1;
        v1 + (v2 << 1) + (v3 << 2) + (v4 << 3)
    }
}
