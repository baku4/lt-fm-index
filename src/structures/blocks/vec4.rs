macro_rules! impl_vec4 {
    // Struct Name, Vector Type
    ( $sn:ident, $vt:ty ) => {
        #[inline]
        fn vectorize(text_chunk: &[u8], rank_pre_counts: &mut Vec<u64>) -> Self {
            let mut bwt_vectors = [0; 4];
            text_chunk.iter().for_each(|chridxwp| {
                let chridx = chridxwp - 1;
                rank_pre_counts[chridx as usize] += 1;
                bwt_vectors[0] <<= 1;
                if chridx & 0b0001 != 0 {
                    bwt_vectors[0] += 1;
                }
                bwt_vectors[1] <<= 1;
                if chridx & 0b0010 != 0 {
                    bwt_vectors[1] += 1;
                }
                bwt_vectors[2] <<= 1;
                if chridx & 0b0100 != 0 {
                    bwt_vectors[2] += 1;
                }
                bwt_vectors[3] <<= 1;
                if chridx & 0b0100 != 0 {
                    bwt_vectors[3] += 1;
                }
                bwt_vectors[4] <<= 1;
                if chridx & 0b1000 != 0 {
                    bwt_vectors[4] += 1;
                }
            });
            Self(bwt_vectors)
        }
        #[inline]
        fn get_remain_count_of(&self, rem: u64, chridx: u8) -> u64 {
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
            count_bits >>= (Self::BLOCK_LEN - rem) as u64;
            count_bits.count_ones() as _
        }
        #[inline]
        fn get_chridx_of(&self, rem: u64) -> u8 {
            let mov = Self::BLOCK_LEN - rem - 1;
            let v1 = (self.0[0] >> mov as u64) as u8 & 1;
            let v2 = (self.0[1] >> mov as u64) as u8 & 1;
            let v3 = (self.0[2] >> mov as u64) as u8 & 1;
            let v4 = (self.0[3] >> mov as u64) as u8 & 1;
            v1 + (v2 << 1) + (v3 << 2) + (v4 << 3)
        }
    };
}
