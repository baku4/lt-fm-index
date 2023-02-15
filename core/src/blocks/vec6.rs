macro_rules! impl_vec6 {
    // Struct Name, Vector Type
    ( $sn:ident, $vt:ty ) => {
        #[inline]
        fn vectorize(text_chunk: &[u8], rank_pre_counts: &mut Vec<TextLen>) -> Self {
            let mut bwt_vectors = [0; 6];
            text_chunk.iter().for_each(|chridxwp| {
                let chridx = chridxwp - 1;
                rank_pre_counts[chridx as usize] += 1;
                bwt_vectors[0] <<= 1;
                if chridx & 0b000001 != 0 {
                    bwt_vectors[0] += 1;
                }
                bwt_vectors[1] <<= 1;
                if chridx & 0b000010 != 0 {
                    bwt_vectors[1] += 1;
                }
                bwt_vectors[2] <<= 1;
                if chridx & 0b000100 != 0 {
                    bwt_vectors[2] += 1;
                }
                bwt_vectors[3] <<= 1;
                if chridx & 0b001000 != 0 {
                    bwt_vectors[3] += 1;
                }
                bwt_vectors[4] <<= 1;
                if chridx & 0b010000 != 0 {
                    bwt_vectors[4] += 1;
                }
                bwt_vectors[5] <<= 1;
                if chridx & 0b100000 != 0 {
                    bwt_vectors[5] += 1;
                }
            });
            Self(bwt_vectors)
        }
        #[inline]
        fn get_remain_count_of(&self, rem: TextLen, chridx: u8) -> TextLen {
            let mut count_bits = match chridx {
                0 => !self.0[5] & !self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & !self.0[0], // 000000
                1 => !self.0[5] & !self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & self.0[0],  // 000001
                2 => !self.0[5] & !self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & !self.0[0],  // 000010
                3 => !self.0[5] & !self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & self.0[0],   // 000011
                4 => !self.0[5] & !self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & !self.0[0],  // 000100
                5 => !self.0[5] & !self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & self.0[0],   // 000101
                6 => !self.0[5] & !self.0[4] & !self.0[3] & self.0[2] & self.0[1] & !self.0[0],   // 000110
                7 => !self.0[5] & !self.0[4] & !self.0[3] & self.0[2] & self.0[1] & self.0[0],    // 000111
                8 => !self.0[5] & !self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & !self.0[0],  // 001000
                9 => !self.0[5] & !self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & self.0[0],   // 001001
                10 => !self.0[5] & !self.0[4] & self.0[3] & !self.0[2] & self.0[1] & !self.0[0],  // 001010
                11 => !self.0[5] & !self.0[4] & self.0[3] & !self.0[2] & self.0[1] & self.0[0],   // 001011
                12 => !self.0[5] & !self.0[4] & self.0[3] & self.0[2] & !self.0[1] & !self.0[0],  // 001100
                13 => !self.0[5] & !self.0[4] & self.0[3] & self.0[2] & !self.0[1] & self.0[0],   // 001101
                14 => !self.0[5] & !self.0[4] & self.0[3] & self.0[2] & self.0[1] & !self.0[0],   // 001110
                15 => !self.0[5] & !self.0[4] & self.0[3] & self.0[2] & self.0[1] & self.0[0],    // 001111
                16 => !self.0[5] & self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & !self.0[0], // 010000
                17 => !self.0[5] & self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & self.0[0],  // 010001
                18 => !self.0[5] & self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & !self.0[0],  // 010010
                19 => !self.0[5] & self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & self.0[0],   // 010011
                20 => !self.0[5] & self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & !self.0[0],  // 010100
                21 => !self.0[5] & self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & self.0[0],   // 010101
                22 => !self.0[5] & self.0[4] & !self.0[3] & self.0[2] & self.0[1] & !self.0[0],   // 010110
                23 => !self.0[5] & self.0[4] & !self.0[3] & self.0[2] & self.0[1] & self.0[0],    // 010111
                24 => !self.0[5] & self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & !self.0[0],  // 011000
                25 => !self.0[5] & self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & self.0[0],   // 011001
                26 => !self.0[5] & self.0[4] & self.0[3] & !self.0[2] & self.0[1] & !self.0[0],   // 011010
                27 => !self.0[5] & self.0[4] & self.0[3] & !self.0[2] & self.0[1] & self.0[0],    // 011011
                28 => !self.0[5] & self.0[4] & self.0[3] & self.0[2] & !self.0[1] & !self.0[0],   // 011100
                29 => !self.0[5] & self.0[4] & self.0[3] & self.0[2] & !self.0[1] & self.0[0],    // 011101
                30 => !self.0[5] & self.0[4] & self.0[3] & self.0[2] & self.0[1] & !self.0[0],    // 011110
                31 => !self.0[5] & self.0[4] & self.0[3] & self.0[2] & self.0[1] & self.0[0],     // 011111
                32 => self.0[5] & !self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & !self.0[0], // 100000
                33 => self.0[5] & !self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & self.0[0],  // 100001
                34 => self.0[5] & !self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & !self.0[0],  // 100010
                35 => self.0[5] & !self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & self.0[0],   // 100011
                36 => self.0[5] & !self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & !self.0[0],  // 100100
                37 => self.0[5] & !self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & self.0[0],   // 100101
                38 => self.0[5] & !self.0[4] & !self.0[3] & self.0[2] & self.0[1] & !self.0[0],   // 100110
                39 => self.0[5] & !self.0[4] & !self.0[3] & self.0[2] & self.0[1] & self.0[0],    // 100111
                40 => self.0[5] & !self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & !self.0[0],  // 101000
                41 => self.0[5] & !self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & self.0[0],   // 101001
                42 => self.0[5] & !self.0[4] & self.0[3] & !self.0[2] & self.0[1] & !self.0[0],   // 101010
                43 => self.0[5] & !self.0[4] & self.0[3] & !self.0[2] & self.0[1] & self.0[0],    // 101011
                44 => self.0[5] & !self.0[4] & self.0[3] & self.0[2] & !self.0[1] & !self.0[0],   // 101100
                45 => self.0[5] & !self.0[4] & self.0[3] & self.0[2] & !self.0[1] & self.0[0],    // 101101
                46 => self.0[5] & !self.0[4] & self.0[3] & self.0[2] & self.0[1] & !self.0[0],    // 101110
                47 => self.0[5] & !self.0[4] & self.0[3] & self.0[2] & self.0[1] & self.0[0],     // 101111
                48 => self.0[5] & self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & !self.0[0],  // 110000
                49 => self.0[5] & self.0[4] & !self.0[3] & !self.0[2] & !self.0[1] & self.0[0],   // 110001
                50 => self.0[5] & self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & !self.0[0],   // 110010
                51 => self.0[5] & self.0[4] & !self.0[3] & !self.0[2] & self.0[1] & self.0[0],    // 110011
                52 => self.0[5] & self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & !self.0[0],   // 110100
                53 => self.0[5] & self.0[4] & !self.0[3] & self.0[2] & !self.0[1] & self.0[0],    // 110101
                54 => self.0[5] & self.0[4] & !self.0[3] & self.0[2] & self.0[1] & !self.0[0],    // 110110
                55 => self.0[5] & self.0[4] & !self.0[3] & self.0[2] & self.0[1] & self.0[0],     // 110111
                56 => self.0[5] & self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & !self.0[0],   // 111000
                57 => self.0[5] & self.0[4] & self.0[3] & !self.0[2] & !self.0[1] & self.0[0],    // 111001
                58 => self.0[5] & self.0[4] & self.0[3] & !self.0[2] & self.0[1] & !self.0[0],    // 111010
                59 => self.0[5] & self.0[4] & self.0[3] & !self.0[2] & self.0[1] & self.0[0],     // 111011
                60 => self.0[5] & self.0[4] & self.0[3] & self.0[2] & !self.0[1] & !self.0[0],    // 111100
                61 => self.0[5] & self.0[4] & self.0[3] & self.0[2] & !self.0[1] & self.0[0],     // 111101
                62 => self.0[5] & self.0[4] & self.0[3] & self.0[2] & self.0[1] & !self.0[0],     // 111110
                _ => self.0[5] & self.0[4] & self.0[3] & self.0[2] & self.0[1] & self.0[0],       // 111111
                
            };
            count_bits >>= (Self::BLOCK_LEN - rem);
            count_bits.count_ones() as _
        }
        #[inline]
        fn get_chridx_of(&self, rem: TextLen) -> u8 {
            let mov = Self::BLOCK_LEN - rem - 1;
            let v1 = (self.0[0] >> mov) as u8 & 1;
            let v2 = (self.0[1] >> mov) as u8 & 1;
            let v3 = (self.0[2] >> mov) as u8 & 1;
            let v4 = (self.0[3] >> mov) as u8 & 1;
            let v5 = (self.0[4] >> mov) as u8 & 1;
            let v6 = (self.0[5] >> mov) as u8 & 1;
            v1 + (v2 << 1) + (v3 << 2) + (v4 << 3) + (v5 << 4) + (v6 << 5)
        }
    };
}
