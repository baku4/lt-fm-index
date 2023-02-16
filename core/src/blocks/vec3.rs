macro_rules! impl_vec3 {
    // Struct Name, Vector Type
    ( $sn:ident, $vt:ty ) => {
        #[inline]
        fn vectorize(text_chunk: &[u8], rank_pre_counts: &mut Vec<TextLength>) -> Self {
            let mut bwt_vectors = [0; 3];
            text_chunk.iter().for_each(|chridxwp| {
                let chridx = chridxwp - 1;
                rank_pre_counts[chridx as usize] += 1;
                bwt_vectors[0] <<= 1;
                if chridx & 0b001 != 0 {
                    bwt_vectors[0] += 1;
                }
                bwt_vectors[1] <<= 1;
                if chridx & 0b010 != 0 {
                    bwt_vectors[1] += 1;
                }
                bwt_vectors[2] <<= 1;
                if chridx & 0b100 != 0 {
                    bwt_vectors[2] += 1;
                }
            });
            Self(bwt_vectors)
        }
        #[inline]
        fn get_remain_count_of(&self, rem: TextLength, chridx: u8) -> TextLength {
            let mut count_bits = match chridx {
                0 => !self.0[2] & !self.0[1] & !self.0[0], // 000
                1 => !self.0[2] & !self.0[1] & self.0[0],  // 001
                2 => !self.0[2] & self.0[1] & !self.0[0],  // 010
                3 => !self.0[2] & self.0[1] & self.0[0],   // 011
                4 => self.0[2] & !self.0[1] & !self.0[0],  // 100
                5 => self.0[2] & !self.0[1] & self.0[0],   // 101
                6 => self.0[2] & self.0[1] & !self.0[0],   // 110
                _ => self.0[2] & self.0[1] & self.0[0],    // 111
            };
            count_bits >>= Self::BLOCK_LEN - rem;
            count_bits.count_ones() as _
        }
        #[inline]
        fn get_chridx_of(&self, rem: TextLength) -> u8 {
            let mov = Self::BLOCK_LEN - rem - 1;
            let v1 = (self.0[0] >> mov as u64) as u8 & 1;
            let v2 = (self.0[1] >> mov as u64) as u8 & 1;
            let v3 = (self.0[2] >> mov as u64) as u8 & 1;
            v1 + (v2 << 1) + (v3 << 2)
        }
    };
}
