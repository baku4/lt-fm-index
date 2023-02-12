use crate::core::Text;
use super::BwtBlock;
use bytemuck::{Pod, Zeroable};

#[macro_use]
mod vec2;
#[macro_use]
mod vec3;
#[macro_use]
mod vec4;
#[macro_use]
mod vec5;

macro_rules! Block {
    // Struct Name, Vector Type, Vector Count
    ( $sn:ident, $vc:tt, $vt:ty ) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
        pub struct $sn([$vt; $vc]);

        impl BwtBlock for $sn {
            const BLOCK_LEN: u64 = <$vt>::BITS as u64;
            
            #[inline]
            fn empty() -> Self {
                Self::zeroed() as Self
            }
            #[inline]
            fn shift_last_offset(&mut self, offset: usize) {
                self.0.iter_mut().for_each(|bits| *bits <<= offset);
            }

            impl_bwt_block!($sn, $vc, $vt);
        }
    };
}

macro_rules! impl_bwt_block {
    ( $sn:ident, 2, $vt:ty ) => {
        impl_vec2!($sn, $vt);
    };
    ( $sn:ident, 3, $vt:ty ) => {
        impl_vec3!($sn, $vt);
    };
    ( $sn:ident, 4, $vt:ty ) => {
        impl_vec4!($sn, $vt);
    };
    ( $sn:ident, 5, $vt:ty ) => {
        impl_vec5!($sn, $vt);
    };
}

Block!(V2u32, 2, u32);
Block!(V2u64, 2, u64);
Block!(V2u128, 2, u128);
Block!(V3u32, 3, u32);
Block!(V3u64, 3, u64);
Block!(V3u128, 3, u128);
Block!(V4u32, 4, u32);
Block!(V4u64, 4, u64);
Block!(V4u128, 4, u128);
Block!(V5u32, 5, u32);
Block!(V5u64, 5, u64);
Block!(V5u128, 5, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
    pub struct TestBlockV2U64([u64; 2]);

    impl BwtBlock for TestBlockV2U64 {
        const BLOCK_LEN: u64 = 64;

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
        fn empty() -> Self {
            Self::zeroed() as Self
        }
        fn shift_last_offset(&mut self, offset: usize) {
            self.0.iter_mut().for_each(|bits| *bits <<= offset);
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
    }
}
