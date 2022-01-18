use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
};
use super::{
    TextEncoder, BwtBlockConstructor, BwtBlockInterface,
    POS_BIT_64, POS_BIT_128,
};

const CHR_COUNT: usize = 21;
const BITS_COUNT: usize = 5;

const A_UTF8: u8 = 65;
const C_UTF8: u8 = 67;
const D_UTF8: u8 = 68;
const E_UTF8: u8 = 69;
const F_UTF8: u8 = 70;
const G_UTF8: u8 = 71;
const H_UTF8: u8 = 72;
const I_UTF8: u8 = 73;
const K_UTF8: u8 = 75;
const L_UTF8: u8 = 76;
const M_UTF8: u8 = 77;
const N_UTF8: u8 = 78;
const P_UTF8: u8 = 80;
const Q_UTF8: u8 = 81;
const R_UTF8: u8 = 82;
const S_UTF8: u8 = 83;
const T_UTF8: u8 = 84;
const V_UTF8: u8 = 86;
const W_UTF8: u8 = 87;
const Y_UTF8: u8 = 89;
const NOISE_UTF8: u8 = 95; // '_' in ASCII

const A_IDX: usize = 0;
const C_IDX: usize = 1;
const D_IDX: usize = 2;
const E_IDX: usize = 3;
const F_IDX: usize = 4;
const G_IDX: usize = 5;
const H_IDX: usize = 6;
const I_IDX: usize = 7;
const K_IDX: usize = 8;
const L_IDX: usize = 9;
const M_IDX: usize = 10;
const N_IDX: usize = 11;
const P_IDX: usize = 12;
const Q_IDX: usize = 13;
const R_IDX: usize = 14;
const S_IDX: usize = 15;
const T_IDX: usize = 16;
const V_IDX: usize = 17;
const W_IDX: usize = 18;
const Y_IDX: usize = 19;
const NOISE_IDX: usize = 20;

const A_IDX_WP: u32 = 1;
const C_IDX_WP: u32 = 2;
const D_IDX_WP: u32 = 3;
const E_IDX_WP: u32 = 4;
const F_IDX_WP: u32 = 5;
const G_IDX_WP: u32 = 6;
const H_IDX_WP: u32 = 7;
const I_IDX_WP: u32 = 8;
const K_IDX_WP: u32 = 9;
const L_IDX_WP: u32 = 10;
const M_IDX_WP: u32 = 11;
const N_IDX_WP: u32 = 12;
const P_IDX_WP: u32 = 13;
const Q_IDX_WP: u32 = 14;
const R_IDX_WP: u32 = 15;
const S_IDX_WP: u32 = 16;
const T_IDX_WP: u32 = 17;
const V_IDX_WP: u32 = 18;
const W_IDX_WP: u32 = 19;
const Y_IDX_WP: u32 = 20;
const NOISE_IDX_WP: u32 = 21;

// * Vector table for Bwt
//
//             -------------------------
//             | 0 | 0 | 0 | 1 | 1 | 1 | 3rd
//             | 0 | 0 | 1 | 0 | 1 | 1 | 4th
//  1st 2nd    | 0 | 1 | 0 | 1 | 0 | 1 | 5th
// -------------------------------------
// | 1 | 1 | - | A | C | D | E | F | G |
// | 1 | 0 | - | H | I | K | L | M | N |
// | 0 | 1 | - | P | Q | R | S | T | V |
// | 0 | 0 | - | W | Y | _ |           |
// -------------------------------------

pub struct TextEncoderAN;

impl TextEncoder for TextEncoderAN {
    const CHR_COUNT: usize = CHR_COUNT;

    fn get_chridx_with_encoding_chr(unencoded_chr_utf8: &mut u8) -> usize {
        match *unencoded_chr_utf8 {
            A_UTF8 => A_IDX,
            C_UTF8 => C_IDX,
            D_UTF8 => D_IDX,
            E_UTF8 => E_IDX,
            F_UTF8 => F_IDX,
            G_UTF8 => G_IDX,
            H_UTF8 => H_IDX,
            I_UTF8 => I_IDX,
            K_UTF8 => K_IDX,
            L_UTF8 => L_IDX,
            M_UTF8 => M_IDX,
            N_UTF8 => N_IDX,
            P_UTF8 => P_IDX,
            Q_UTF8 => Q_IDX,
            R_UTF8 => R_IDX,
            S_UTF8 => S_IDX,
            T_UTF8 => T_IDX,
            V_UTF8 => V_IDX,
            W_UTF8 => W_IDX,
            Y_UTF8 => Y_IDX,
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
            D_UTF8 => D_IDX,
            E_UTF8 => E_IDX,
            F_UTF8 => F_IDX,
            G_UTF8 => G_IDX,
            H_UTF8 => H_IDX,
            I_UTF8 => I_IDX,
            K_UTF8 => K_IDX,
            L_UTF8 => L_IDX,
            M_UTF8 => M_IDX,
            N_UTF8 => N_IDX,
            P_UTF8 => P_IDX,
            Q_UTF8 => Q_IDX,
            R_UTF8 => R_IDX,
            S_UTF8 => S_IDX,
            T_UTF8 => T_IDX,
            V_UTF8 => V_IDX,
            W_UTF8 => W_IDX,
            Y_UTF8 => Y_IDX,
            _ => NOISE_IDX,
        }
    }
    fn chrwpidx_of_chr(chr: u8) -> u32 {
        match chr {
            A_UTF8 => A_IDX_WP,
            C_UTF8 => C_IDX_WP,
            D_UTF8 => D_IDX_WP,
            E_UTF8 => E_IDX_WP,
            F_UTF8 => F_IDX_WP,
            G_UTF8 => G_IDX_WP,
            H_UTF8 => H_IDX_WP,
            I_UTF8 => I_IDX_WP,
            K_UTF8 => K_IDX_WP,
            L_UTF8 => L_IDX_WP,
            M_UTF8 => M_IDX_WP,
            N_UTF8 => N_IDX_WP,
            P_UTF8 => P_IDX_WP,
            Q_UTF8 => Q_IDX_WP,
            R_UTF8 => R_IDX_WP,
            S_UTF8 => S_IDX_WP,
            T_UTF8 => T_IDX_WP,
            V_UTF8 => V_IDX_WP,
            W_UTF8 => W_IDX_WP,
            Y_UTF8 => Y_IDX_WP,
            _ => NOISE_IDX_WP,
        }
    }
}


// To use Rust type inference, copy the code without specifying trait for u64, u128 primitive.

#[derive(Archive, Serialize, Deserialize)]
#[archive(archived = "BwtBlock64AN")]
pub struct BwtBlock64ANPreBuild {
    rank_check_point: [u64; CHR_COUNT],
    bwt_vector: [u64; BITS_COUNT],
}

impl BwtBlockConstructor for BwtBlock64ANPreBuild {
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
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                },
                C_UTF8 => {
                    rank_check_point[C_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[4] += 1;
                },
                D_UTF8 => {
                    rank_check_point[D_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[3] += 1;
                },
                E_UTF8 => {
                    rank_check_point[E_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[4] += 1;
                },
                F_UTF8 => {
                    rank_check_point[F_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                },
                G_UTF8 => {
                    rank_check_point[G_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                    bwt_vector[4] += 1;
                },
                H_UTF8 => {
                    rank_check_point[H_IDX] += 1;
                    bwt_vector[0] += 1;
                },
                I_UTF8 => {
                    rank_check_point[I_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[4] += 1;
                },
                K_UTF8 => {
                    rank_check_point[K_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[3] += 1;
                },
                L_UTF8 => {
                    rank_check_point[L_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[4] += 1;
                },
                M_UTF8 => {
                    rank_check_point[M_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                },
                N_UTF8 => {
                    rank_check_point[N_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                    bwt_vector[4] += 1;
                },
                P_UTF8 => {
                    rank_check_point[P_IDX] += 1;
                    bwt_vector[1] += 1;
                },
                Q_UTF8 => {
                    rank_check_point[Q_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[4] += 1;
                },
                R_UTF8 => {
                    rank_check_point[R_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[3] += 1;
                },
                S_UTF8 => {
                    rank_check_point[S_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[4] += 1;
                },
                T_UTF8 => {
                    rank_check_point[T_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                },
                V_UTF8 => {
                    rank_check_point[V_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                    bwt_vector[4] += 1;
                },
                W_UTF8 => {
                    rank_check_point[W_IDX] += 1;
                },
                Y_UTF8 => {
                    rank_check_point[Y_IDX] += 1;
                    bwt_vector[4] += 1;
                }
                _ => { // NOISE
                    rank_check_point[NOISE_IDX] += 1;
                    bwt_vector[3] += 1;
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

impl BwtBlockInterface for BwtBlock64AN {
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_64; // TO CHANGE
        pos_bit >>= rem;

        let chridx = if self.bwt_vector[0] & pos_bit == 0 {
            if self.bwt_vector[1] & pos_bit == 0 {
                // 00
                if self.bwt_vector[3] & pos_bit == 0 {
                    if self.bwt_vector[4] & pos_bit == 0 {
                        // 00 ?00
                        W_IDX
                    } else {
                        // 00 ?01
                        Y_IDX
                    }
                } else {
                    // 00 ?1?
                    NOISE_IDX
                }
            } else {
                // 01
                if self.bwt_vector[2] & pos_bit == 0 {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 01 000
                            P_IDX
                        } else {
                            // 01 001
                            Q_IDX
                        }
                    } else {
                        // 01 01?
                        R_IDX
                    }
                } else {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        // 01 10?
                        S_IDX
                    } else {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 01 110
                            T_IDX
                        } else {
                            // 01 111
                            V_IDX
                        }
                    }
                }
            }
        } else {
            if self.bwt_vector[1] & pos_bit == 0 {
                // 10
                if self.bwt_vector[2] & pos_bit == 0 {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 10 000
                            H_IDX
                        } else {
                            // 10 001
                            I_IDX
                        }
                    } else {
                        // 10 01?
                        K_IDX
                    }
                } else {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        // 10 10?
                        L_IDX
                    } else {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 10 110
                            M_IDX
                        } else {
                            // 10 111
                            N_IDX
                        }
                    }
                }
            } else {
                // 11
                if self.bwt_vector[2] & pos_bit == 0 {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 11 000
                            A_IDX
                        } else {
                            // 11 001
                            C_IDX
                        }
                    } else {
                        // 11 01?
                        D_IDX
                    }
                } else {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        // 11 10?
                        E_IDX
                    } else {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 11 110
                            F_IDX
                        } else {
                            // 11 111
                            G_IDX
                        }
                    }
                }
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
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[3] &
                    !self.bwt_vector[4]
                },
                C_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[4]
                },
                D_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[3]
                },
                E_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[3]
                },
                F_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[4]
                },
                G_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[3] &
                    self.bwt_vector[4]
                },
                H_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    !self.bwt_vector[3] &
                    !self.bwt_vector[4]
                },
                I_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[4]
                },
                K_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[3]
                },
                L_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[3]
                },
                M_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[4]
                },
                N_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[3] &
                    self.bwt_vector[4]
                },
                P_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[3] &
                    !self.bwt_vector[4]
                },
                Q_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[4]
                },
                R_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[3]
                },
                S_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[3]
                },
                T_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[4]
                },
                V_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[3] &
                    self.bwt_vector[4]
                },
                W_IDX => {
                    !self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    !self.bwt_vector[3] &
                    !self.bwt_vector[4]
                },
                Y_IDX => {
                    !self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[4]
                }
                _ => { // NOISE
                    !self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[3]
                }
            } >> BwtBlock64ANPreBuild::BLOCK_SEG_LEN-rem; // TO CHANGE
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}

#[derive(Archive, Serialize, Deserialize)]
#[archive(archived = "BwtBlock128AN")]
pub struct BwtBlock128ANPreBuild {
    rank_check_point: [u64; CHR_COUNT],
    bwt_vector: [u128; BITS_COUNT],
}

impl BwtBlockConstructor for BwtBlock128ANPreBuild {
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
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                },
                C_UTF8 => {
                    rank_check_point[C_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[4] += 1;
                },
                D_UTF8 => {
                    rank_check_point[D_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[3] += 1;
                },
                E_UTF8 => {
                    rank_check_point[E_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[4] += 1;
                },
                F_UTF8 => {
                    rank_check_point[F_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                },
                G_UTF8 => {
                    rank_check_point[G_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                    bwt_vector[4] += 1;
                },
                H_UTF8 => {
                    rank_check_point[H_IDX] += 1;
                    bwt_vector[0] += 1;
                },
                I_UTF8 => {
                    rank_check_point[I_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[4] += 1;
                },
                K_UTF8 => {
                    rank_check_point[K_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[3] += 1;
                },
                L_UTF8 => {
                    rank_check_point[L_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[4] += 1;
                },
                M_UTF8 => {
                    rank_check_point[M_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                },
                N_UTF8 => {
                    rank_check_point[N_IDX] += 1;
                    bwt_vector[0] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                    bwt_vector[4] += 1;
                },
                P_UTF8 => {
                    rank_check_point[P_IDX] += 1;
                    bwt_vector[1] += 1;
                },
                Q_UTF8 => {
                    rank_check_point[Q_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[4] += 1;
                },
                R_UTF8 => {
                    rank_check_point[R_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[3] += 1;
                },
                S_UTF8 => {
                    rank_check_point[S_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[4] += 1;
                },
                T_UTF8 => {
                    rank_check_point[T_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                },
                V_UTF8 => {
                    rank_check_point[V_IDX] += 1;
                    bwt_vector[1] += 1;
                    bwt_vector[2] += 1;
                    bwt_vector[3] += 1;
                    bwt_vector[4] += 1;
                },
                W_UTF8 => {
                    rank_check_point[W_IDX] += 1;
                },
                Y_UTF8 => {
                    rank_check_point[Y_IDX] += 1;
                    bwt_vector[4] += 1;
                }
                _ => { // NOISE
                    rank_check_point[NOISE_IDX] += 1;
                    bwt_vector[3] += 1;
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

impl BwtBlockInterface for BwtBlock128AN {
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_128; // TO CHANGE
        pos_bit >>= rem;

        let chridx = if self.bwt_vector[0] & pos_bit == 0 {
            if self.bwt_vector[1] & pos_bit == 0 {
                // 00
                if self.bwt_vector[3] & pos_bit == 0 {
                    if self.bwt_vector[4] & pos_bit == 0 {
                        // 00 ?00
                        W_IDX
                    } else {
                        // 00 ?01
                        Y_IDX
                    }
                } else {
                    // 00 ?1?
                    NOISE_IDX
                }
            } else {
                // 01
                if self.bwt_vector[2] & pos_bit == 0 {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 01 000
                            P_IDX
                        } else {
                            // 01 001
                            Q_IDX
                        }
                    } else {
                        // 01 01?
                        R_IDX
                    }
                } else {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        // 01 10?
                        S_IDX
                    } else {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 01 110
                            T_IDX
                        } else {
                            // 01 111
                            V_IDX
                        }
                    }
                }
            }
        } else {
            if self.bwt_vector[1] & pos_bit == 0 {
                // 10
                if self.bwt_vector[2] & pos_bit == 0 {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 10 000
                            H_IDX
                        } else {
                            // 10 001
                            I_IDX
                        }
                    } else {
                        // 10 01?
                        K_IDX
                    }
                } else {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        // 10 10?
                        L_IDX
                    } else {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 10 110
                            M_IDX
                        } else {
                            // 10 111
                            N_IDX
                        }
                    }
                }
            } else {
                // 11
                if self.bwt_vector[2] & pos_bit == 0 {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 11 000
                            A_IDX
                        } else {
                            // 11 001
                            C_IDX
                        }
                    } else {
                        // 11 01?
                        D_IDX
                    }
                } else {
                    if self.bwt_vector[3] & pos_bit == 0 {
                        // 11 10?
                        E_IDX
                    } else {
                        if self.bwt_vector[4] & pos_bit == 0 {
                            // 11 110
                            F_IDX
                        } else {
                            // 11 111
                            G_IDX
                        }
                    }
                }
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
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[3] &
                    !self.bwt_vector[4]
                },
                C_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[4]
                },
                D_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[3]
                },
                E_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[3]
                },
                F_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[4]
                },
                G_IDX => {
                    self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[3] &
                    self.bwt_vector[4]
                },
                H_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    !self.bwt_vector[3] &
                    !self.bwt_vector[4]
                },
                I_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[4]
                },
                K_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[3]
                },
                L_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[3]
                },
                M_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[4]
                },
                N_IDX => {
                    self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[3] &
                    self.bwt_vector[4]
                },
                P_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[3] &
                    !self.bwt_vector[4]
                },
                Q_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[4]
                },
                R_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    !self.bwt_vector[2] &
                    self.bwt_vector[3]
                },
                S_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[3]
                },
                T_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[2] &
                    !self.bwt_vector[4]
                },
                V_IDX => {
                    !self.bwt_vector[0] &
                    self.bwt_vector[1] &
                    self.bwt_vector[3] &
                    self.bwt_vector[4]
                },
                W_IDX => {
                    !self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    !self.bwt_vector[3] &
                    !self.bwt_vector[4]
                },
                Y_IDX => {
                    !self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[4]
                }
                _ => { // NOISE
                    !self.bwt_vector[0] &
                    !self.bwt_vector[1] &
                    self.bwt_vector[3]
                }
            } >> BwtBlock128ANPreBuild::BLOCK_SEG_LEN-rem; // TO CHANGE
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}
