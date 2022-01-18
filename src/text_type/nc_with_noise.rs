use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
};
use super::{
    TextEncoder, BwtBlockConstructor, BwtBlockInterface,
    POS_BIT_64, POS_BIT_128,
};

const CHR_COUNT: usize = 5;
const BITS_COUNT: usize = 3;

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

const A_IDX_WP: u32 = 1;
const C_IDX_WP: u32 = 2;
const G_IDX_WP: u32 = 3;
const T_IDX_WP: u32 = 4;
const NOISE_IDX_WP: u32 = 5;

// * Vector table for Bwt
// | A | C | G | T | _ |
// | 0 | 0 | 1 | 1 | 0 | first
// | 0 | 1 | 0 | 1 | 0 | second
// | 1 | 0 | 1 | 0 | 0 | third

pub struct TextEncoderNN;

impl TextEncoder for TextEncoderNN {
    const CHR_COUNT: usize = CHR_COUNT;

    fn get_chridx_with_encoding_chr(unencoded_chr_utf8: &mut u8) -> usize {
        match *unencoded_chr_utf8 {
            A_UTF8 => A_IDX,
            C_UTF8 => C_IDX,
            G_UTF8 => G_IDX,
            T_UTF8 => T_IDX,
            _ => {
                *unencoded_chr_utf8 = NOISE_UTF8;
                NOISE_IDX
            },
        }
    }
    fn chridx_of_chr(chr: u8) -> usize {
        match chr {
            A_UTF8 => A_IDX,
            C_UTF8 => C_IDX,
            G_UTF8 => G_IDX,
            T_UTF8 => T_IDX,
            _ => NOISE_IDX,
        }
    }
    fn chrwpidx_of_chr(chr: u8) -> u32 {
        match chr {
            A_UTF8 => A_IDX_WP,
            C_UTF8 => C_IDX_WP,
            G_UTF8 => G_IDX_WP,
            T_UTF8 => T_IDX_WP,
            _ => NOISE_IDX_WP,
        }
    }
}


// To use Rust type inference, copy the code without specifying trait for u64, u128 primitive.

#[derive(Archive, Serialize, Deserialize)]
#[archive(archived = "BwtBlock64NN")]
pub struct BwtBlock64NNPreBuild {
    rank_check_point: [u64; CHR_COUNT],
    bwt_vector: [u64; BITS_COUNT],
}

impl BwtBlockConstructor for BwtBlock64NNPreBuild {
    const BLOCK_SEG_LEN: u64 = 64; // TO CHANGE
    
    type RankCheckPoint = [u64; CHR_COUNT];
    type BwtVector = [u64; BITS_COUNT]; // TO CHANGE

    fn empty_rank_check_point() -> Self::RankCheckPoint {
        [0; CHR_COUNT]
    }
    fn encoding_text_chunk(text_chunk: &[u8], rank_check_point: &mut Self::RankCheckPoint) -> Self::BwtVector {
        let mut bwt_vector = [0; BITS_COUNT];

        text_chunk.iter().for_each(|c| {
            bwt_vector.iter_mut().for_each(|bwt_bits| *bwt_bits <<= 1);
            match *c {
                A_UTF8 => {
                    rank_check_point[A_IDX] += 1;
                    bwt_vector[2] += 1;
                },
                C_UTF8 => {
                    rank_check_point[C_IDX] += 1;
                    bwt_vector[1] += 1;
                },
                G_UTF8 => {
                    rank_check_point[G_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[2] += 1;
                },
                T_UTF8 => {
                    rank_check_point[T_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                },
                _ => { // NOISE
                    rank_check_point[NOISE_IDX] += 1;
                }
            }
        });

        bwt_vector
    }
    fn new(block_rank_check_point: Self::RankCheckPoint, bwt_vector: Self::BwtVector) -> Self {
        Self {
            rank_check_point: block_rank_check_point,
            bwt_vector,
        }
    }
    fn new_last(rank_check_point: Self::RankCheckPoint) -> Self {
        Self {
            rank_check_point,
            bwt_vector: [0; BITS_COUNT],
        }
    }
    fn add_offset(&mut self, last_offset: usize) {
        self.bwt_vector.iter_mut().for_each(|bwt_bits| *bwt_bits <<= last_offset);
    }
}

impl BwtBlockInterface for BwtBlock64NN {
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_64; // TO CHANGE
        pos_bit >>= rem;

        let chridx = if self.bwt_vector[0] & pos_bit == 0 {
            if self.bwt_vector[1] & pos_bit == 0 {
                if self.bwt_vector[2] & pos_bit == 0 {
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
            if self.bwt_vector[1] & pos_bit == 0 {
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
        let mut rank = self.rank_check_point[chridx];

        if rem != 0 {
            let count_bits = match chridx {
                A_IDX => {
                    (!self.bwt_vector[0] & self.bwt_vector[2]) >> BwtBlock64NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                C_IDX => {
                    (!self.bwt_vector[0] & self.bwt_vector[1]) >> BwtBlock64NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                G_IDX => {
                    (self.bwt_vector[0] & self.bwt_vector[2]) >> BwtBlock64NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                T_IDX => {
                    (self.bwt_vector[0] & self.bwt_vector[1]) >> BwtBlock64NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                _ => { // NOISE
                    (!self.bwt_vector[1] & !self.bwt_vector[2]) >> BwtBlock64NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                }
            };
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}

#[derive(Archive, Serialize, Deserialize)]
#[archive(archived = "BwtBlock128NN")]
pub struct BwtBlock128NNPreBuild {
    rank_check_point: [u64; CHR_COUNT],
    bwt_vector: [u128; BITS_COUNT],
}

impl BwtBlockConstructor for BwtBlock128NNPreBuild {
    const BLOCK_SEG_LEN: u64 = 128; // TO CHANGE
    
    type RankCheckPoint = [u64; CHR_COUNT];
    type BwtVector = [u128; BITS_COUNT]; // TO CHANGE

    fn empty_rank_check_point() -> Self::RankCheckPoint {
        [0; CHR_COUNT]
    }
    fn encoding_text_chunk(text_chunk: &[u8], rank_check_point: &mut Self::RankCheckPoint) -> Self::BwtVector {
        let mut bwt_vector = [0; BITS_COUNT];

        text_chunk.iter().for_each(|c| {
            bwt_vector.iter_mut().for_each(|bwt_bits| *bwt_bits <<= 1);
            match *c {
                A_UTF8 => {
                    rank_check_point[A_IDX] += 1;
                    bwt_vector[2] += 1;
                },
                C_UTF8 => {
                    rank_check_point[C_IDX] += 1;
                    bwt_vector[1] += 1;
                },
                G_UTF8 => {
                    rank_check_point[G_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[2] += 1;
                },
                T_UTF8 => {
                    rank_check_point[T_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                },
                _ => { // NOISE
                    rank_check_point[NOISE_IDX] += 1;
                }
            }
        });

        bwt_vector
    }
    fn new(block_rank_check_point: Self::RankCheckPoint, bwt_vector: Self::BwtVector) -> Self {
        Self {
            rank_check_point: block_rank_check_point,
            bwt_vector,
        }
    }
    fn new_last(rank_check_point: Self::RankCheckPoint) -> Self {
        Self {
            rank_check_point,
            bwt_vector: [0; BITS_COUNT],
        }
    }
    fn add_offset(&mut self, last_offset: usize) {
        self.bwt_vector.iter_mut().for_each(|bwt_bits| *bwt_bits <<= last_offset);
    }
}

impl BwtBlockInterface for BwtBlock128NN {
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_128; // TO CHANGE
        pos_bit >>= rem;

        let chridx = if self.bwt_vector[0] & pos_bit == 0 {
            if self.bwt_vector[1] & pos_bit == 0 {
                if self.bwt_vector[2] & pos_bit == 0 {
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
            if self.bwt_vector[1] & pos_bit == 0 {
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
        let mut rank = self.rank_check_point[chridx];

        if rem != 0 {
            let count_bits = match chridx {
                A_IDX => {
                    (!self.bwt_vector[0] & self.bwt_vector[2]) >> BwtBlock128NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                C_IDX => {
                    (!self.bwt_vector[0] & self.bwt_vector[1]) >> BwtBlock128NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                G_IDX => {
                    (self.bwt_vector[0] & self.bwt_vector[2]) >> BwtBlock128NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                T_IDX => {
                    (self.bwt_vector[0] & self.bwt_vector[1]) >> BwtBlock128NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                _ => { // NOISE
                    (!self.bwt_vector[1] & !self.bwt_vector[2]) >> BwtBlock128NNPreBuild::BLOCK_SEG_LEN-rem // TO CHANGE
                }
            };
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}
