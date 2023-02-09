use super::{
    TextEncoder,
    BwtBlock,
};

mod blocks;
use blocks::*; // All is blocks

#[derive(Debug)]
pub struct ChrCountError;

macro_rules! make_text_encoder {
    ( $name: ident, $chr: tt,  $bits: tt ) => {
        pub struct $name {
            chr_count: usize,
            table: [u8; 256],
        }
        impl TextEncoder for $name {
            type BwtBlockType = appropriate_block_type!($chr, $bits);

            fn chr_count(&self) -> usize {
                self.chr_count
            }
            fn chr_idx_table(&self) -> [u8; 256] {
                self.table
            }
        }
        impl $name {
            impl_new!($chr);
        }
    };
}
macro_rules! appropriate_block_type {
    ( 2,  32 ) => { B3U64 };
    ( 2,  64 ) => { B3U64 };
    ( 2,  128 ) => { B3U128 };
    ( 3,  32 ) => { B4U64 };
    ( 3,  64 ) => { B4U64 };
    ( 3,  128 ) => { B4U128 };
}
macro_rules! impl_new {
    ( $chr:expr ) => {
        pub fn new(chrs: &[&[u8]; $chr]) -> Self {
            let mut table = [$chr; 256];
            chrs.iter().enumerate().for_each(|(idx, chr)| {
                chr.iter().for_each(|x| table[*x as usize] = idx as u8);
            });

            Self { chr_count: $chr+1, table }
        }
        pub fn from_vec_slices(chrs: &[Vec<u8>]) -> Result<Self, ChrCountError> {
            let mut table = [$chr; 256];
            if chrs.len() != $chr {
                return Err(ChrCountError)
            }
            chrs.iter().enumerate().for_each(|(idx, chr)| {
                chr.iter().for_each(|x| table[*x as usize] = idx as u8);
            });

            Ok(Self { chr_count: $chr+1, table })
        }
    };
}

make_text_encoder!(C3B32, 3, 32);
make_text_encoder!(C3B64, 3, 64);
make_text_encoder!(C3B128, 3, 128);
