const SEG_LEN: u64 = 64;
use crate::utils::BIT_COUNT_TABLE;

// * VECTOR TABLE
// | A | C | G | T |
// | 0 | 0 | 1 | 1 | first
// | 0 | 1 | 0 | 1 | second

#[allow(unused_imports)]
use super::{A_UTF8, C_UTF8, G_UTF8, T_UTF8, A_U8_IDX, C_U8_IDX, G_U8_IDX, T_U8_IDX};
use super::CountArray;
use super::nc_to_idx;
use serde::{Serialize, Deserialize};

use std::u64;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[repr(align(64))]
struct BwtBlock {
    rank_checkpoint: [u64; 4],
    first_bwt_vector: u64,
    second_bwt_vector: u64,
}

impl BwtBlock {
    #[inline]
    fn get_rank_from_chr(&self, rem: &u64, c: &u8) -> u64 {
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
    #[inline]
    fn get_rank_and_chr(&self, rem: u64) -> (u8, u64) {
        let cidx = self.get_chr(&rem);
        if rem == 0 {
            (cidx, self.rank_checkpoint[cidx as usize])
        } else {
            let rank = self.get_rank_from_chr_idx(&rem, &cidx);
            (cidx, rank)
        }
    }
    #[inline]
    fn get_chr(&self, rem: &u64) -> u8 {
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
    fn get_rank_from_chr_idx(&self, rem: &u64, cidx: &u8) -> u64 {
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
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BwtOn {
    primary_index: u64,
    blocks: Vec<BwtBlock>,
}

impl BwtOn {
    #[inline]
    pub fn new(bwt_string: Vec<u8>, primary_index: i64) -> Self {
        let mut chunk_size = bwt_string.len()/64;
        let last_offset = {
            let rem = bwt_string.len()%64;
            if rem == 0 {
                rem
            } else {
                chunk_size += 1;
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
    pub fn next_pos_range_from_range(&self, pos_range: (u64, u64), c: u8, count_array: &CountArray) -> (u64, u64) {
        (
            self.next_pos_from_chr(pos_range.0, &c, count_array),
            self.next_pos_from_chr(pos_range.1, &c, count_array),
        )
    }
    #[inline]
    pub fn next_pos_from_chr(&self, mut pos: u64, c: &u8, count_array: &CountArray) -> u64 {
        if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos/SEG_LEN;
        let rem = pos%SEG_LEN;
        if rem == 0 {
            count_array[nc_to_idx(&c)] + self.blocks[quot as usize].rank_checkpoint[nc_to_idx(&c)]
        } else {
            let rank = self.blocks[quot as usize].get_rank_from_chr(&rem, &c);
            count_array[nc_to_idx(&c)] + rank
        }
    }
    #[inline]
    pub fn get_pre_pos(&self, mut pos: u64, count_array: &CountArray) -> Option<u64> {
        if pos == self.primary_index - 1 {
            return None;
        } else if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos/SEG_LEN;
        let rem = pos%SEG_LEN;
        let (cidx, rank) = self.blocks[quot as usize].get_rank_and_chr(rem);
        Some(count_array[cidx as usize] + rank)
    }
}
