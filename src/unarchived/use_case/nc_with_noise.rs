use super::{Deserialize, Serialize};
use super::{Text, Pattern};

use super::{CountArray, CountArrayProto, BwtBlock, POS_BIT_64, POS_BIT_128};

const CHR_COUNT: usize = 5;
const CHR_WITH_PIDX_COUNT: usize = CHR_COUNT + 1;

const A_UTF8: u8 = 65;
const C_UTF8: u8 = 67;
const G_UTF8: u8 = 71;
const T_UTF8: u8 = 84;
const NOISE_UTF8: u8 = 95; // '_' in ASCII

const A_IDX: usize = 0;
const C_IDX: usize = 1;
const G_IDX: usize = 2;
const T_IDX: usize = 3;
const NOISE_IDX: usize = 4;

const A_IDX_WP: usize = A_IDX + 1;
const C_IDX_WP: usize = C_IDX + 1;
const G_IDX_WP: usize = G_IDX + 1;
const T_IDX_WP: usize = T_IDX + 1;
const NOISE_IDX_WP: usize = NOISE_IDX + 1;

// * Vector table for Bwt
// | A | C | G | T | _ |
// | 0 | 0 | 1 | 1 | 0 | first
// | 0 | 1 | 0 | 1 | 0 | second
// | 1 | 0 | 1 | 0 | 0 | third

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CountArrayNN {
    proto: CountArrayProto,
}

impl CountArray for CountArrayNN {
    fn new_and_encode_text(text: &mut Text, kmer_size: usize) -> Self {
        let proto = CountArrayProto::new_and_encode_text(
            text,
            kmer_size,
            CHR_COUNT,
            CHR_WITH_PIDX_COUNT,
            Self::get_chridx_with_encoding_chr,
        );
        Self { proto }
    }
    fn get_precount_of_chridx(&self, chridx: usize) -> u64 {
        self.proto.get_precount_of_chridx(chridx)
    }
    fn get_chridx_and_precount_of_chr(&self, chr: u8) -> (usize, u64) {
        self.proto.get_chridx_and_precount_of_chr(chr, Self::chridx_of_chr)
    }
    fn get_initial_pos_range_and_idx_of_pattern(&self, pattern: Pattern) -> ((u64, u64), usize) {
        self.proto.get_initial_pos_range_and_idx_of_pattern(pattern, Self::chrwpidx_of_chr)
    }
    fn kmer_size(&self) -> usize {
        self.proto.kmer_size()
    }
}

impl CountArrayNN {
    fn chridx_of_chr(chr: u8) -> usize {
        match chr {
            A_UTF8 => A_IDX,
            C_UTF8 => C_IDX,
            G_UTF8 => G_IDX,
            T_UTF8 => T_IDX,
            _ => NOISE_IDX,
        }
    }
    fn chrwpidx_of_chr(chr: u8) -> usize {
        match chr {
            A_UTF8 => A_IDX_WP,
            C_UTF8 => C_IDX_WP,
            G_UTF8 => G_IDX_WP,
            T_UTF8 => T_IDX_WP,
            _ => NOISE_IDX_WP,
        }
    }
    fn get_chridx_with_encoding_chr(chr: &mut u8) -> usize {
        match *chr {
            A_UTF8 => A_IDX,
            C_UTF8 => C_IDX,
            G_UTF8 => G_IDX,
            T_UTF8 => T_IDX,
            _ => {
                *chr = NOISE_UTF8;
                NOISE_IDX
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BwtBlock64NN {
    rank_checkpoint: [u64; CHR_COUNT],
    first_bwt_vector: u64,
    second_bwt_vector: u64,
    third_bwt_vector: u64,
}

impl BwtBlock for BwtBlock64NN {
    const BLOCK_SEG_LEN: u64 = 64;

    fn new_with_bwt_text(bwt_text: Text) -> Vec<Self> {
        let mut chunk_count = bwt_text.len() / Self::BLOCK_SEG_LEN as usize;
        let rem = bwt_text.len() % Self::BLOCK_SEG_LEN as usize;
        
        let last_offset = if rem == 0 {
            chunk_count += 1;
            rem
        } else {
            Self::BLOCK_SEG_LEN as usize - rem
        };

        let mut rank_checkpoint: [u64; CHR_COUNT] = [0; CHR_COUNT];
        let mut blocks: Vec<Self> = Vec::with_capacity(chunk_count);

        bwt_text.chunks(Self::BLOCK_SEG_LEN as usize).for_each(|string_chunk| {
            let block_checkpoint = rank_checkpoint.clone();

            let mut first_bwt_vector = 0;
            let mut second_bwt_vector = 0;
            let mut third_bwt_vector = 0;

            for c in string_chunk {
                first_bwt_vector <<= 1;
                second_bwt_vector <<= 1;
                third_bwt_vector <<= 1;
                match *c {
                    A_UTF8 => {
                        rank_checkpoint[A_IDX] += 1;
                        third_bwt_vector += 1;
                    },
                    C_UTF8 => {
                        rank_checkpoint[C_IDX] += 1;
                        second_bwt_vector += 1;
                    },
                    G_UTF8 => {
                        rank_checkpoint[G_IDX] += 1;
                        first_bwt_vector += 1;
                        third_bwt_vector += 1;
                    },
                    T_UTF8 => {
                        rank_checkpoint[T_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                    },
                    _ => { // NOISE
                        rank_checkpoint[NOISE_IDX] += 1;
                    }
                }
            }
            let block = Self {
                rank_checkpoint: block_checkpoint,
                first_bwt_vector: first_bwt_vector,
                second_bwt_vector: second_bwt_vector,
                third_bwt_vector,
            };
            
            blocks.push(block);
        });

        if last_offset == 0 {
            let last_block = Self {
                rank_checkpoint: rank_checkpoint,
                first_bwt_vector: 0,
                second_bwt_vector: 0,
                third_bwt_vector: 0,
            };
            blocks.push(last_block);
        } else {
            let last_block = blocks.last_mut().unwrap();
            last_block.add_offset(last_offset);
        }

        blocks
    }
    fn add_offset(&mut self, offset: usize) {
        self.first_bwt_vector <<= offset;
        self.second_bwt_vector <<= offset;
        self.third_bwt_vector <<= offset;
    }
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_64;
        pos_bit >>= rem;

        let chridx = if self.first_bwt_vector & pos_bit == 0 {
            if self.second_bwt_vector & pos_bit == 0 {
                if self.third_bwt_vector & pos_bit == 0 {
                    // 000
                    NOISE_IDX
                } else {
                    // 001
                    A_IDX
                }
            } else {
                // 01?
                C_IDX
            }
        } else {
            if self.second_bwt_vector & pos_bit == 0 {
                // 10?
                G_IDX
            } else {
                // 11?
                T_IDX
            }
        };

        let rank = self.get_rank_of_chridx_and_rem(chridx, rem);

        (chridx, rank)
    }
    fn get_rank_of_chridx_and_rem(&self, chridx: usize, rem: u64) -> u64 {
        let mut rank = self.rank_checkpoint[chridx];

        if rem != 0 {
            let count_bits = match chridx {
                A_IDX => {
                    (!self.first_bwt_vector & self.third_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                },
                C_IDX => {
                    (!self.first_bwt_vector & self.second_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                },
                G_IDX => {
                    (self.first_bwt_vector & self.third_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                },
                T_IDX => {
                    (self.first_bwt_vector & self.second_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                },
                _ => { // NOISE
                    (!self.second_bwt_vector & !self.third_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                }
            };
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}



#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BwtBlock128NN {
    rank_checkpoint: [u64; CHR_COUNT],
    first_bwt_vector: u128,
    second_bwt_vector: u128,
    third_bwt_vector: u128,
}

impl BwtBlock for BwtBlock128NN {
    const BLOCK_SEG_LEN: u64 = 128;

    fn new_with_bwt_text(bwt_text: Text) -> Vec<Self> {
        let mut chunk_count = bwt_text.len() / Self::BLOCK_SEG_LEN as usize;
        let rem = bwt_text.len() % Self::BLOCK_SEG_LEN as usize;
        
        let last_offset = if rem == 0 {
            chunk_count += 1;
            rem
        } else {
            Self::BLOCK_SEG_LEN as usize - rem
        };

        let mut rank_checkpoint: [u64; CHR_COUNT] = [0; CHR_COUNT];
        let mut blocks: Vec<Self> = Vec::with_capacity(chunk_count);

        bwt_text.chunks(Self::BLOCK_SEG_LEN as usize).for_each(|string_chunk| {
            let block_checkpoint = rank_checkpoint.clone();

            let mut first_bwt_vector = 0;
            let mut second_bwt_vector = 0;
            let mut third_bwt_vector = 0;

            for c in string_chunk {
                first_bwt_vector <<= 1;
                second_bwt_vector <<= 1;
                third_bwt_vector <<= 1;
                match *c {
                    A_UTF8 => {
                        rank_checkpoint[A_IDX] += 1;
                        third_bwt_vector += 1;
                    },
                    C_UTF8 => {
                        rank_checkpoint[C_IDX] += 1;
                        second_bwt_vector += 1;
                    },
                    G_UTF8 => {
                        rank_checkpoint[G_IDX] += 1;
                        first_bwt_vector += 1;
                        third_bwt_vector += 1;
                    },
                    T_UTF8 => {
                        rank_checkpoint[T_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                    },
                    _ => { // NOISE
                        rank_checkpoint[NOISE_IDX] += 1;
                    }
                }
            }
            let block = Self {
                rank_checkpoint: block_checkpoint,
                first_bwt_vector: first_bwt_vector,
                second_bwt_vector: second_bwt_vector,
                third_bwt_vector,
            };
            
            blocks.push(block);
        });

        if last_offset == 0 {
            let last_block = Self {
                rank_checkpoint: rank_checkpoint,
                first_bwt_vector: 0,
                second_bwt_vector: 0,
                third_bwt_vector: 0,
            };
            blocks.push(last_block);
        } else {
            let last_block = blocks.last_mut().unwrap();
            last_block.add_offset(last_offset);
        }

        blocks
    }
    fn add_offset(&mut self, offset: usize) {
        self.first_bwt_vector <<= offset;
        self.second_bwt_vector <<= offset;
        self.third_bwt_vector <<= offset;
    }
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_128;
        pos_bit >>= rem;

        let chridx = if self.first_bwt_vector & pos_bit == 0 {
            if self.second_bwt_vector & pos_bit == 0 {
                if self.third_bwt_vector & pos_bit == 0 {
                    NOISE_IDX
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
        };

        let rank = self.get_rank_of_chridx_and_rem(chridx, rem);

        (chridx, rank)
    }
    fn get_rank_of_chridx_and_rem(&self, chridx: usize, rem: u64) -> u64 {
        let mut rank = self.rank_checkpoint[chridx];

        if rem != 0 {
            let count_bits = match chridx {
                A_IDX => {
                    (!self.first_bwt_vector & self.third_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                },
                C_IDX => {
                    (!self.first_bwt_vector & self.second_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                },
                G_IDX => {
                    (self.first_bwt_vector & self.third_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                },
                T_IDX => {
                    (self.first_bwt_vector & self.second_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                },
                _ => { // NOISE
                    (!self.second_bwt_vector & !self.third_bwt_vector) >> Self::BLOCK_SEG_LEN-rem
                }
            };
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}