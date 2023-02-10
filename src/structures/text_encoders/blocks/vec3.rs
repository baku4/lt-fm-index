macro_rules! Vec3 {
    ( $name: ident, $chr: tt,  $bits: ty )  => {
        impl $name {
            #[inline]
            fn vectorize(
                text_chunk: &[u8],
                rank_checkpoint: &mut [u64; $chr],
                bwt_vector: &mut [$bits; 3],
            ) {
                text_chunk.iter().for_each(|chridxwp| {
                    let chridx = chridxwp - 1;
                    rank_checkpoint[chridx as usize] += 1;
                    bwt_vector[0] <<= 1;
                    if chridx & 0b001 != 0 {
                        bwt_vector[0] += 1;
                    }
                    bwt_vector[1] <<= 1;
                    if chridx & 0b010 != 0 {
                        bwt_vector[1] += 1;
                    }
                    bwt_vector[2] <<= 1;
                    if chridx & 0b100 != 0 {
                        bwt_vector[2] += 1;
                    }
                });
            }
            #[inline]
            fn get_rank_inner(&self, rem: u64, chridx: u8) -> u64 {
                let mut rank = self.rank_checkpoint[chridx as usize];
                if rem != 0 {
                    let mut count_bits: $bits = match chridx {
                        0 => !self.bwt_vector[2] & !self.bwt_vector[1] & !self.bwt_vector[0], // 000
                        1 => !self.bwt_vector[2] & !self.bwt_vector[1] & self.bwt_vector[0],  // 001
                        2 => !self.bwt_vector[2] & self.bwt_vector[1] & !self.bwt_vector[0],  // 010
                        3 => !self.bwt_vector[2] & self.bwt_vector[1] & self.bwt_vector[0],   // 011
                        4 => self.bwt_vector[2] & !self.bwt_vector[1] & !self.bwt_vector[0],  // 100
                        5 => self.bwt_vector[2] & !self.bwt_vector[1] & self.bwt_vector[0],   // 101
                        6 => self.bwt_vector[2] & self.bwt_vector[1] & !self.bwt_vector[0],   // 110
                        _ => self.bwt_vector[2] & self.bwt_vector[1] & self.bwt_vector[0],    // 111
                    };
                    count_bits >>= (Self::BIT_LEN - rem) as $bits;
                    rank += count_bits.count_ones() as u64;
                };
        
                rank
            }
            #[inline]
            fn get_rank_and_chridx_of_rem_inner(&self, rem: u64) -> (u64, u8) {
                let mov = (Self::BIT_LEN - rem - 1);
                let v1 = (self.bwt_vector[0] >> mov as $bits) as u8 & 1;
                let v2 = (self.bwt_vector[1] >> mov as $bits) as u8 & 1 ;
                let v3 = (self.bwt_vector[2] >> mov as $bits) as u8 & 1 ;
                let chridx = v1 + (v2 << 1) + (v2 << 2);
                let rank = self.get_rank_inner(rem, chridx);
        
                (rank, chridx)
            }
        }
    };
}
