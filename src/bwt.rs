const SEG_LEN: u64 = 64;
const BIT_COUNT_TABLE: [u64; 256] = [
    0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
    4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
];

use super::{A_UTF8, C_UTF8, G_UTF8, T_UTF8, nc_to_idx};
use super::{A_U8_IDX, C_U8_IDX, G_U8_IDX, T_U8_IDX};
use super::CountArray;
use std::u64;

#[repr(align(64))]
struct BwtBlock {
    rank_checkpoint: [u64; 4],
    first_bwt_vector: u64,
    second_bwt_vector: u64,
}

impl BwtBlock {
    fn new() {

    }
    #[inline]
    fn get_first_rank_of_c(&self, rem: &u64, c: &u8) {

    }
    #[inline]
    fn get_rank_and_c(&self, rem: u64) -> (u8, u64) {
        let c = self.get_c(&rem);
        let rank = self.get_rank(&rem, &c);
        (c, rank)
    }
    #[inline]
    fn get_c(&self, rem: &u64) -> u8 {
        let mut pos_bit: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        pos_bit >>= rem-1;
        if self.first_bwt_vector & pos_bit == 0 {
            if self.second_bwt_vector & pos_bit == 0 {
                // 00
                A_U8_IDX
            } else {
                // 01
                C_U8_IDX
            }
        } else {
            if self.second_bwt_vector & pos_bit == 0 {
                // 10
                G_U8_IDX
            } else {
                // 11
                T_U8_IDX
            }
        }
    }
    #[inline]
    fn get_rank(&self, rem: &u64, c: &u8) -> u64 {
        let count_bits = match *c {
            A_UTF8 => {
                (self.first_bwt_vector & u64::MIN) & (self.second_bwt_vector & u64::MIN) >> 64_u64-rem
            },
            C_UTF8 => {
                (self.first_bwt_vector & u64::MIN) & (self.second_bwt_vector & u64::MAX) >> 64_u64-rem
            },
            G_UTF8 => {
                (self.first_bwt_vector & u64::MAX) & (self.second_bwt_vector & u64::MIN) >> 64_u64-rem
            },
            _ => {
                (self.first_bwt_vector & u64::MAX) & (self.second_bwt_vector & u64::MAX) >> 64_u64-rem
            }
        };
        let mut rank: u64 = 0;
        count_bits.to_ne_bytes().iter().for_each(|&byte| rank += BIT_COUNT_TABLE[byte as usize]);
        rank
    }
}

pub struct Bwt {
    primary_index: u64,
    blocks: Vec<BwtBlock>,
}

impl Bwt {
    #[inline]
    pub fn new(bwt_string: Vec<u8>, primary_index: i64) {
        
    }
    #[inline]
    pub fn lf_map_with_range(&self, pos_range: (u64, u64), c: u8, count_array: &CountArray) -> (u64, u64) {
        let sp = {
            let quot = pos_range.0/SEG_LEN;
            let rem = pos_range.0%SEG_LEN;
            let rank = self.blocks[quot as usize].get_rank(&rem, &c);
            count_array[nc_to_idx(&c)] + rank
        };
        let ep = {
            let quot = pos_range.1/SEG_LEN;
            let rem = pos_range.1%SEG_LEN;
            let rank = self.blocks[quot as usize].get_rank(&rem, &c);
            count_array[nc_to_idx(&c)] + rank
        };
        (sp, ep)
    }
    #[inline]
    pub fn lf_map_with_pos(&self, pos: u64, count_array: &CountArray) -> u64 {
        let quot = pos/SEG_LEN;
        let rem = pos%SEG_LEN;
        let (cidx, rank) = self.blocks[quot as usize].get_rank_and_c(rem);
        count_array[cidx as usize] + rank
    }
}