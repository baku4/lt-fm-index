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
                    let mut count_bits: $bits = count_bits_of_chridx!(self, chridx, $chr);
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

macro_rules! count_bits_of_chridx {
    ( $self:ident, $chridx:ident, 1 ) => {
        match $chridx {
            _ => match_return!($self, 0),
        }
    };
    ( $self:ident, $chridx:ident, 2 ) => {
        match $chridx {
            0 => match_return!($self, 0),
            _ => match_return!($self, 1),
        }
    };
    ( $self:ident, $chridx:ident, 3 ) => {
        match $chridx {
            0 => match_return!($self, 0),
            1 => match_return!($self, 1),
            _ => match_return!($self, 2),
        }
    };
    ( $self:ident, $chridx:ident, 4 ) => {
        match $chridx {
            0 => match_return!($self, 0),
            1 => match_return!($self, 1),
            2 => match_return!($self, 2),
            _ => match_return!($self, 3),
        }
    };
}
macro_rules! match_return {
    ( $self:ident, 0 ) => {
        !$self.bwt_vector[1] & !$self.bwt_vector[0]
    };
    ( $self:ident, 1 ) => {
        !$self.bwt_vector[1] & $self.bwt_vector[0]
    };
    ( $self:ident, 2 ) => {
        $self.bwt_vector[1] & !$self.bwt_vector[0]
    };
    ( $self:ident, 3 ) => {
        $self.bwt_vector[1] & $self.bwt_vector[0]
    };
}
