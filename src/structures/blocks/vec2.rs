macro_rules! impl_vec2 {
    // Struct Name, Vector Type
    ( $sn:ident, $vt:ty ) => {
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
            let v2 = (self.0[1] >> mov as u64) as u8 & 1;
            v1 + (v2 << 1)
        }
    };
}
