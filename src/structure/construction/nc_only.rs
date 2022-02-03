use super::{
    Result,
    EndianType, ReadBytesExt, WriteBytesExt, Serializable,
    TextEncoder, BwtBlockInterface,
    POS_BIT_64, POS_BIT_128,
};

const CHR_COUNT: usize = 4;
const BITS_COUNT: usize = 2;

const A_UTF8: u8 = 65;
const C_UTF8: u8 = 67;
const G_UTF8: u8 = 71;
const NOISE_UTF8: u8 = 84; // 'T' in ASCII

const A_IDX: usize = 0;
const C_IDX: usize = 1;
const G_IDX: usize = 2;
const NOISE_IDX: usize = 3;

const A_IDX_WP: u32 = 1;
const C_IDX_WP: u32 = 2;
const G_IDX_WP: u32 = 3;
const NOISE_IDX_WP: u32 = 4;

// * Vector table for Bwt
// | A | C | G | T |
// | 0 | 0 | 1 | 1 | first
// | 0 | 1 | 0 | 1 | second

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextEncoderNO;

impl TextEncoder for TextEncoderNO {
    const CHR_COUNT: usize = CHR_COUNT;

    fn get_chridx_with_encoding_chr(unencoded_chr_utf8: &mut u8) -> usize {
        match *unencoded_chr_utf8 {
            A_UTF8 => A_IDX,
            C_UTF8 => C_IDX,
            G_UTF8 => G_IDX,
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
            _ => NOISE_IDX,
        }
    }
    fn chrwpidx_of_chr(chr: u8) -> u32 {
        match chr {
            A_UTF8 => A_IDX_WP,
            C_UTF8 => C_IDX_WP,
            G_UTF8 => G_IDX_WP,
            _ => NOISE_IDX_WP,
        }
    }
}


// To use Rust type inference, copy the code without specifying trait for u64, u128 primitive.

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BwtBlock64NO {
    rank_check_point: [u64; CHR_COUNT],
    bwt_vector: [u64; BITS_COUNT],
}

impl BwtBlockInterface for BwtBlock64NO {
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
                },
                C_UTF8 => {
                    rank_check_point[C_IDX] += 1;
                    bwt_vector[1] += 1;
                },
                G_UTF8 => {
                    rank_check_point[G_IDX] += 1;
                    bwt_vector[0] += 1;
                },
                _ => { // NOISE
                    rank_check_point[NOISE_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
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

    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_64; // TO CHANGE
        pos_bit >>= rem;

        let chridx = if self.bwt_vector[0] & pos_bit == 0 {
            if self.bwt_vector[1] & pos_bit == 0 {
                // 00
                A_IDX
            } else {
                // 01
                C_IDX
            }
        } else {
            if self.bwt_vector[1] & pos_bit == 0 {
                // 10
                G_IDX
            } else {
                // 11
                NOISE_IDX
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
                    (!self.bwt_vector[0] & !self.bwt_vector[1]) >> BwtBlock64NO::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                C_IDX => {
                    (!self.bwt_vector[0] & self.bwt_vector[1]) >> BwtBlock64NO::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                G_IDX => {
                    (self.bwt_vector[0] & !self.bwt_vector[1]) >> BwtBlock64NO::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                _ => { // NOISE
                    (self.bwt_vector[0] & self.bwt_vector[1]) >> BwtBlock64NO::BLOCK_SEG_LEN-rem // TO CHANGE
                }
            };
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}

impl Serializable for BwtBlock64NO {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write,
    {
        let casted: &[u64; CHR_COUNT + BITS_COUNT] = bytemuck::cast_ref(self);
        casted.iter().for_each(|v| {
            writer.write_u64::<EndianType>(*v);
        });
        
        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        let mut raw_array: [u64; CHR_COUNT + BITS_COUNT] = [0; CHR_COUNT + BITS_COUNT];

        reader.read_u64_into::<EndianType>(&mut raw_array)?;

        let casted = bytemuck::cast(raw_array);
        
        Ok(casted)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BwtBlock128NO {
    rank_check_point: [u64; CHR_COUNT],
    bwt_vector: [u128; BITS_COUNT],
}

impl BwtBlockInterface for BwtBlock128NO {
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
                },
                C_UTF8 => {
                    rank_check_point[C_IDX] += 1;
                    bwt_vector[1] += 1;
                },
                G_UTF8 => {
                    rank_check_point[G_IDX] += 1;
                    bwt_vector[0] += 1;
                },
                _ => { // NOISE
                    rank_check_point[NOISE_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
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

    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_128; // TO CHANGE
        pos_bit >>= rem;

        let chridx = if self.bwt_vector[0] & pos_bit == 0 {
            if self.bwt_vector[1] & pos_bit == 0 {
                // 00
                A_IDX
            } else {
                // 01
                C_IDX
            }
        } else {
            if self.bwt_vector[1] & pos_bit == 0 {
                // 10
                G_IDX
            } else {
                // 11
                NOISE_IDX
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
                    (!self.bwt_vector[0] & !self.bwt_vector[1]) >> BwtBlock128NO::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                C_IDX => {
                    (!self.bwt_vector[0] & self.bwt_vector[1]) >> BwtBlock128NO::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                G_IDX => {
                    (self.bwt_vector[0] & !self.bwt_vector[1]) >> BwtBlock128NO::BLOCK_SEG_LEN-rem // TO CHANGE
                },
                _ => { // NOISE
                    (self.bwt_vector[0] & self.bwt_vector[1]) >> BwtBlock128NO::BLOCK_SEG_LEN-rem // TO CHANGE
                }
            };
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}

impl Serializable for BwtBlock128NO {
    #[allow(unused_must_use)]
    fn save_to<W>(&self, mut writer: W) -> Result<()> where
        W: std::io::Write,
    {
        let casted: &[u64; CHR_COUNT + (2 * BITS_COUNT)] = bytemuck::cast_ref(self);
        casted.iter().for_each(|v| {
            writer.write_u64::<EndianType>(*v);
        });
        
        Ok(())
    }
    fn load_from<R>(mut reader: R) -> Result<Self> where
        R: std::io::Read,
        Self: Sized,
    {
        let mut raw_array: [u64; CHR_COUNT + (2 * BITS_COUNT)] = [0; CHR_COUNT + (2 * BITS_COUNT)];

        reader.read_u64_into::<EndianType>(&mut raw_array)?;

        let casted = bytemuck::cast(raw_array);
        
        Ok(casted)
    }
}
