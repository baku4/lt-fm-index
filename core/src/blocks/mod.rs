use crate::TextLen;
use super::Block;
use bytemuck::{Pod, Zeroable};

#[macro_use]
mod vec2;
#[macro_use]
mod vec3;
#[macro_use]
mod vec4;
#[macro_use]
mod vec5;
#[macro_use]
mod vec6;

macro_rules! GenBlock {
    // Struct Name, Vector Type, Vector Count
    ( $sn:ident, $vc:tt, $vt:ty ) => {
        #[repr(C)]
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
        pub struct $sn([$vt; $vc]);

        impl Block for $sn {
            const BLOCK_LEN: TextLen = <$vt>::BITS as TextLen;
            
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
    ( $sn:ident, 6, $vt:ty ) => {
        impl_vec6!($sn, $vt);
    };
}

GenBlock!(V2w32, 2, u32);
GenBlock!(V2w64, 2, u64);
GenBlock!(V2w128, 2, u128);
GenBlock!(V3w32, 3, u32);
GenBlock!(V3w64, 3, u64);
GenBlock!(V3w128, 3, u128);
GenBlock!(V4w32, 4, u32);
GenBlock!(V4w64, 4, u64);
GenBlock!(V4w128, 4, u128);
GenBlock!(V5w32, 5, u32);
GenBlock!(V5w64, 5, u64);
GenBlock!(V5w128, 5, u128);
GenBlock!(V6w32, 6, u32);
GenBlock!(V6w64, 6, u64);
GenBlock!(V6w128, 6, u128);

#[cfg(test)]
mod tests {
    use super::*;

    #[repr(C)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Pod, Zeroable)]
    pub struct TestBlockV2U64([u64; 2]);

    impl Block for TestBlockV2U64 {
        const BLOCK_LEN: TextLen = 64;

        #[inline]
        fn vectorize(text_chunk: &[u8], rank_pre_counts: &mut Vec<TextLen>) -> Self {
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
        fn get_remain_count_of(&self, rem: TextLen, chridx: u8) -> TextLen {
            let mut count_bits = match chridx {
                0 => !self.0[1] & !self.0[0], // 00
                1 => !self.0[1] & self.0[0],  // 01
                2 => self.0[1] & !self.0[0],  // 10
                _ => self.0[1] & self.0[0],   // 11
            };
            count_bits >>= (Self::BLOCK_LEN - rem) as TextLen;
            count_bits.count_ones() as _
        }
        #[inline]
        fn get_chridx_of(&self, rem: TextLen) -> u8 {
            let mov = Self::BLOCK_LEN - rem - 1;
            let v1 = (self.0[0] >> mov) as u8 & 1;
            let v2 = (self.0[1] >> mov) as u8 & 1;
            v1 + (v2 << 1)
        }
    }
}
