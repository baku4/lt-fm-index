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

#[allow(unused_imports)]
use super::{A_UTF8, C_UTF8, G_UTF8, T_UTF8, A_U8_IDX, C_U8_IDX, G_U8_IDX, T_U8_IDX};
use super::CountArray;
use super::nc_to_idx;

use std::u64;

#[derive(Debug)]
#[repr(align(64))]
struct BwtBlock {
    rank_checkpoint: [u64; 4],
    first_bwt_vector: u64,
    second_bwt_vector: u64,
}

impl BwtBlock {
    #[inline]
    fn get_rank_and_cidx(&self, rem: u64) -> (u8, u64) {
        let cidx = self.get_cidx(&rem);
        if rem == 0 {
            (cidx, self.rank_checkpoint[cidx as usize])
        } else {
            let rank = self.get_rank_with_cidx(&rem, &cidx);
            (cidx, rank)
        }
    }
    #[inline]
    fn get_cidx(&self, rem: &u64) -> u8 {
        let mut pos_bit: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        pos_bit >>= rem;
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
    fn get_rank_with_cidx(&self, rem: &u64, cidx: &u8) -> u64 {
        let count_bits = match *cidx {
            A_U8_IDX => {
                (!self.first_bwt_vector & !self.second_bwt_vector) >> 64_u64-rem
            },
            C_U8_IDX => {
                (!self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem
            },
            G_U8_IDX => {
                (self.first_bwt_vector & !self.second_bwt_vector) >> 64_u64-rem
            },
            _ => {
                (self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem
            }
        };
        let mut rank: u64 = 0;
        count_bits.to_ne_bytes().iter().for_each(|&byte| rank += BIT_COUNT_TABLE[byte as usize]);
        self.rank_checkpoint[*cidx as usize] + rank
    }
    #[inline]
    fn get_rank_with_c(&self, rem: &u64, c: &u8) -> u64 {
        let (count_bits, rank_chkp) = match *c {
            A_UTF8 => {
                ((!self.first_bwt_vector & !self.second_bwt_vector) >> 64_u64-rem, self.rank_checkpoint[0])
            },
            C_UTF8 => {
                ((!self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem, self.rank_checkpoint[1])
            },
            G_UTF8 => {
                ((self.first_bwt_vector & !self.second_bwt_vector) >> 64_u64-rem, self.rank_checkpoint[2])
            },
            _ => {
                ((self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem, self.rank_checkpoint[3])
            }
        };
        let mut rank: u64 = 0;
        count_bits.to_ne_bytes().iter().for_each(|&byte| rank += BIT_COUNT_TABLE[byte as usize]);
        rank_chkp + rank
    }
}

#[derive(Debug)]
pub struct Bwt {
    primary_index: u64,
    blocks: Vec<BwtBlock>,
}

impl Bwt {
    #[inline]
    pub fn new(bwt_string: Vec<u8>, primary_index: i64) -> Self {
        let chunk_size = bwt_string.len()/64;
        let last_offset = {
            let rem = bwt_string.len()%64;
            if rem == 0 {
                rem
            } else {
                64-rem
            }
        };
        let mut blocks: Vec<BwtBlock> = Vec::with_capacity(chunk_size);
        // push bwt block
        let mut rank_checkpoint: [u64; 4] = [0; 4];
        bwt_string.chunks(64).for_each(|string_chunk| {
            let block_checkpoint = rank_checkpoint.clone();
            let mut first_bwt_vector: u64 = 0;
            let mut second_bwt_vector: u64 = 0;
            for c in string_chunk {
                first_bwt_vector <<= 1;
                second_bwt_vector <<= 1;
                match *c {
                    A_UTF8 => {
                        rank_checkpoint[0] += 1;
                    },
                    C_UTF8 => {
                        rank_checkpoint[1] += 1;
                        second_bwt_vector += 1;
                    },
                    G_UTF8 => {
                        rank_checkpoint[2] += 1;
                        first_bwt_vector += 1;
                    },
                    _ => {
                        rank_checkpoint[3] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                    }
                }
            }
            blocks.push(
                BwtBlock {
                    rank_checkpoint: block_checkpoint,
                    first_bwt_vector: first_bwt_vector,
                    second_bwt_vector: second_bwt_vector,
                }
            );
        });
        // add offset to last block
        let last_block = blocks.last_mut().unwrap();
        last_block.first_bwt_vector <<= last_offset;
        last_block.second_bwt_vector <<= last_offset;
        Self {
            primary_index: primary_index as u64,
            blocks: blocks,
        }
    }
    #[inline]
    pub fn lf_map_with_range(&self, pos_range: (u64, u64), c: u8, count_array: &CountArray) -> (u64, u64) {
        (
            self.lf_map_with_c(pos_range.0, &c, count_array),
            self.lf_map_with_c(pos_range.1, &c, count_array),
        )
    }
    #[inline]
    pub fn lf_map_with_c(&self, mut pos: u64, c: &u8, count_array: &CountArray) -> u64 {
        if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos/SEG_LEN;
        let rem = pos%SEG_LEN;
        if rem == 0 {
            count_array[nc_to_idx(&c)] + self.blocks[quot as usize].rank_checkpoint[nc_to_idx(&c)]
        } else {
            let rank = self.blocks[quot as usize].get_rank_with_c(&rem, &c);
            count_array[nc_to_idx(&c)] + rank
        }
    }
    #[inline]
    pub fn lf_map_with_pos(&self, mut pos: u64, count_array: &CountArray) -> u64 {
        if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos/SEG_LEN;
        let rem = pos%SEG_LEN;
        let (cidx, rank) = self.blocks[quot as usize].get_rank_and_cidx(rem);
        count_array[cidx as usize] + rank
    }
}
