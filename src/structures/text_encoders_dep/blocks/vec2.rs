macro_rules! Vec2 {
    ( $name: ident, $chr: tt,  $bits: ty )  => {
        impl $name {
            #[inline]
            fn vectorize(
                text_chunk: &[u8],
                rank_checkpoint: &mut [u64; $chr],
                bwt_vector: &mut [$bits; 2],
            ) {
                text_chunk.iter().for_each(|chridxwp| {
                    let chridx = chridxwp - 1;
                    rank_checkpoint[chridx as usize] += 1;
                    bwt_vector[0] <<= 1;
                    if chridx & 0b01 != 0 {
                        bwt_vector[0] += 1;
                    }
                    bwt_vector[1] <<= 1;
                    if chridx & 0b10 != 0 {
                        bwt_vector[1] += 1;
                    }
                });
            }
            #[inline]
            fn get_rank_inner(&self, rem: u64, chridx: u8) -> u64 {
                let mut rank = self.rank_checkpoint[chridx as usize];
                if rem != 0 {
                    let mut count_bits: $bits = match chridx {
                        0 => !self.bwt_vector[1] & !self.bwt_vector[0], // 00
                        1 => !self.bwt_vector[1] & self.bwt_vector[0],  // 01
                        2 => self.bwt_vector[1] & !self.bwt_vector[0],  // 10
                        _ => self.bwt_vector[1] & self.bwt_vector[0],   // 11
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
                let chridx = v1 + (v2 << 1);
                let rank = self.get_rank_inner(rem, chridx);
        
                (rank, chridx)
            }
        }
    };
}
