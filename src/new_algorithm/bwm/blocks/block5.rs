use crate::core::Position;
use super::{Block, Vector};

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Block5<V: Vector>([V; 5]);

impl<V: Vector> Block for Block5<V> {
    const BLOCK_LEN: u32 = V::BLOCK_LEN;
    const MAX_CHR: u32 = 31;

    #[inline]
    fn vectorize<P: Position>(text_chunk: &[u8], rank_pre_counts: &mut Vec<P>) -> Self {
        let mut bwt_vectors = [V::ZERO; 5];
        text_chunk.iter().for_each(|chridxwp| {
            let chridx = chridxwp - 1;
            rank_pre_counts[chridx as usize] += P::ONE;
            bwt_vectors[0] <<= V::ONE;
            if chridx & 0b00001 != 0 {
                bwt_vectors[0] += V::ONE;
            }
            bwt_vectors[1] <<= V::ONE;
            if chridx & 0b00010 != 0 {
                bwt_vectors[1] += V::ONE;
            }
            bwt_vectors[2] <<= V::ONE;
            if chridx & 0b00100 != 0 {
                bwt_vectors[2] += V::ONE;
            }
            bwt_vectors[3] <<= V::ONE;
            if chridx & 0b01000 != 0 {
                bwt_vectors[3] += V::ONE;
            }
            bwt_vectors[4] <<= V::ONE;
            if chridx & 0b10000 != 0 {
                bwt_vectors[4] += V::ONE;
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
            0 => !self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & !self.0[0], // 00000
            1 => !self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & self.0[0],  // 00001
            2 => !self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & !self.0[0],  // 00010
            3 => !self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & self.0[0],   // 00011
            4 => !self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & !self.0[0],  // 00100
            5 => !self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & self.0[0],   // 00101
            6 => !self.0[4] & !self.0[3] & self.0[2] & self.0[1] & !self.0[0],   // 00110
            7 => !self.0[4] & !self.0[3] & self.0[2] & self.0[1] & self.0[0],    // 00111
            8 => !self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & !self.0[0],  // 01000
            9 => !self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & self.0[0],   // 01001
            10 => !self.0[4] & self.0[3] & !self.0[2] & self.0[1] & !self.0[0],  // 01010
            11 => !self.0[4] & self.0[3] & !self.0[2] & self.0[1] & self.0[0],   // 01011
            12 => !self.0[4] & self.0[3] & self.0[2] & !self.0[1] & !self.0[0],  // 01100
            13 => !self.0[4] & self.0[3] & self.0[2] & !self.0[1] & self.0[0],   // 01101
            14 => !self.0[4] & self.0[3] & self.0[2] & self.0[1] & !self.0[0],   // 01110
            15 => !self.0[4] & self.0[3] & self.0[2] & self.0[1] & self.0[0],    // 01111
            16 => self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & !self.0[0], // 10000
            17 => self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & self.0[0],  // 10001
            18 => self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & !self.0[0],  // 10010
            19 => self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & self.0[0],   // 10011
            20 => self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & !self.0[0],  // 10100
            21 => self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & self.0[0],   // 10101
            22 => self.0[4] & !self.0[3] & self.0[2] & self.0[1] & !self.0[0],   // 10110
            23 => self.0[4] & !self.0[3] & self.0[2] & self.0[1] & self.0[0],    // 10111
            24 => self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & !self.0[0],  // 11000
            25 => self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & self.0[0],   // 11001
            26 => self.0[4] & self.0[3] & !self.0[2] & self.0[1] & !self.0[0],   // 11010
            27 => self.0[4] & self.0[3] & !self.0[2] & self.0[1] & self.0[0],    // 11011
            28 => self.0[4] & self.0[3] & self.0[2] & !self.0[1] & !self.0[0],   // 11100
            29 => self.0[4] & self.0[3] & self.0[2] & !self.0[1] & self.0[0],    // 11101
            30 => self.0[4] & self.0[3] & self.0[2] & self.0[1] & !self.0[0],    // 11110
            _ => self.0[4] & self.0[3] & self.0[2] & self.0[1] & self.0[0],      // 11111
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
        let v5 = (self.0[4] >> V::from_u32(mov)).as_u8() & 1;
        v1 + (v2 << 1) + (v3 << 2) + (v4 << 3) + (v5 << 4)
    }
}
