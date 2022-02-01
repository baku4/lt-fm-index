const SEG_LEN: u64 = 64;
use super::super::utils::BIT_COUNT_TABLE;

// * VECTOR TABLE
// | A | C | G | T | N |
// | 0 | 0 | 1 | 1 | 0 | first
// | 0 | 1 | 0 | 1 | 0 | second
// | 1 | 0 | 1 | 0 | 0 | third

#[allow(unused_imports)]
use super::{
    A_UTF8, C_UTF8, G_UTF8, T_UTF8,
    A_U8_IDX, C_U8_IDX, G_U8_IDX, T_U8_IDX, N_U8_IDX,
    A_IDX, C_IDX, G_IDX, T_IDX, N_IDX,
};
use super::CountArray;
use super::nc_to_idx;
use serde::{Serialize, Deserialize};

use std::u64;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[repr(align(64))]
struct BwtBlock {
    rank_checkpoint: RankCheckpoint,
    first_bwt_vector: u64,
    second_bwt_vector: u64,
    third_bwt_vector: u64,
}
type RankCheckpoint = [u64; 5];

impl BwtBlock {
    // For position range search
    #[inline]
    fn get_rank_from_chr(&self, rem: &u64, chr: &u8) -> u64 {
        let (count_bits, rank_chkp) = match *chr {
            A_UTF8 => {
                ((!self.first_bwt_vector & self.third_bwt_vector) >> 64_u64-rem, self.rank_checkpoint[0])
            },
            C_UTF8 => {
                ((!self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem, self.rank_checkpoint[1])
            },
            G_UTF8 => {
                ((self.first_bwt_vector & self.third_bwt_vector) >> 64_u64-rem, self.rank_checkpoint[2])
            },
            T_UTF8 => {
                ((self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem, self.rank_checkpoint[3])
            },
            _ => { // N
                ((!self.second_bwt_vector & !self.third_bwt_vector) >> 64_u64-rem, self.rank_checkpoint[4])
            }
        };
        let mut rank: u64 = 0;
        count_bits.to_ne_bytes().iter().for_each(|&byte| rank += BIT_COUNT_TABLE[byte as usize]);
        rank_chkp + rank
    }
    // Pre-position serach
    #[inline]
    fn get_rank_and_chr(&self, rem: u64) -> (usize, u64) {
        let chr_idx = self.get_chr(&rem);
        if rem == 0 {
            (chr_idx, self.rank_checkpoint[chr_idx])
        } else {
            let rank = self.get_rank_from_chr_idx(&rem, &chr_idx);
            (chr_idx, rank)
        }
    }
    #[inline]
    fn get_chr(&self, rem: &u64) -> usize {
        let mut pos_bit: u64 = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000;
        pos_bit >>= rem;
        // check
        if self.first_bwt_vector & pos_bit == 0 {
            if self.second_bwt_vector & pos_bit == 0 {
                if self.third_bwt_vector & pos_bit == 0 {
                    N_IDX
                } else {
                    A_IDX
                }
            } else {
                C_IDX
            }
        } else {
            if self.second_bwt_vector & pos_bit == 0 {
                G_IDX
            } else {
                T_IDX
            }
        }
    }
    #[inline]
    fn get_rank_from_chr_idx(&self, rem: &u64, chr_idx: &usize) -> u64 {
        let count_bits = match *chr_idx {
            A_IDX => {
                (!self.first_bwt_vector & self.third_bwt_vector) >> 64_u64-rem
            },
            C_IDX => {
                (!self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem
            },
            G_IDX => {
                (self.first_bwt_vector & self.third_bwt_vector) >> 64_u64-rem
            },
            T_IDX => {
                (self.first_bwt_vector & self.second_bwt_vector) >> 64_u64-rem
            },
            _ => { // X
                (!self.second_bwt_vector & !self.third_bwt_vector) >> 64_u64-rem
            }
        };
        let mut rank: u64 = 0;
        count_bits.to_ne_bytes().iter().for_each(|&byte| rank += BIT_COUNT_TABLE[byte as usize]);
        self.rank_checkpoint[*chr_idx as usize] + rank
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BwtNn {
    primary_index: u64,
    blocks: Vec<BwtBlock>,
}

impl BwtNn {
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
        let mut blocks: Vec<BwtBlock> = Vec::with_capacity(chunk_size+1);
        // push bwt block
        let mut rank_checkpoint: RankCheckpoint = [0; 5];
        bwt_string.chunks(64).for_each(|string_chunk| {
            let block_checkpoint = rank_checkpoint.clone();
            let mut first_bwt_vector: u64 = 0;
            let mut second_bwt_vector: u64 = 0;
            let mut third_bwt_bector: u64 = 0;
            for c in string_chunk {
                first_bwt_vector <<= 1;
                second_bwt_vector <<= 1;
                third_bwt_bector <<= 1;
                match *c {
                    A_UTF8 => {
                        rank_checkpoint[0] += 1;
                        third_bwt_bector += 1;
                    },
                    C_UTF8 => {
                        rank_checkpoint[1] += 1;
                        second_bwt_vector += 1;
                    },
                    G_UTF8 => {
                        rank_checkpoint[2] += 1;
                        first_bwt_vector += 1;
                        third_bwt_bector += 1;
                    },
                    T_UTF8 => {
                        rank_checkpoint[3] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                    },
                    _ => { // N
                        rank_checkpoint[4] += 1;
                    }
                }
            }
            blocks.push(
                BwtBlock {
                    rank_checkpoint: block_checkpoint,
                    first_bwt_vector: first_bwt_vector,
                    second_bwt_vector: second_bwt_vector,
                    third_bwt_vector: third_bwt_bector,
                }
            );
        });
        // add offset to last block
        let last_block = blocks.last_mut().unwrap();
        last_block.first_bwt_vector <<= last_offset;
        last_block.second_bwt_vector <<= last_offset;
        last_block.third_bwt_vector <<= last_offset;
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
        let (ca_idx, rank) = self.blocks[quot as usize].get_rank_and_chr(rem);
        Some(count_array[ca_idx as usize] + rank)
    }
}
