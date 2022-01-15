use crate::{Deserialize, Serialize};
use crate::{Text, Pattern};

use super::{CountArray, CountArrayProto, BwtBlock, POS_BIT_64, POS_BIT_128};

const CHR_COUNT: usize = 21;
const CHR_WITH_PIDX_COUNT: usize = CHR_COUNT + 1;

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

const A_IDX_WP: usize = A_IDX + 1;
const C_IDX_WP: usize = C_IDX + 1;
const D_IDX_WP: usize = D_IDX + 1;
const E_IDX_WP: usize = E_IDX + 1;
const F_IDX_WP: usize = F_IDX + 1;
const G_IDX_WP: usize = G_IDX + 1;
const H_IDX_WP: usize = H_IDX + 1;
const I_IDX_WP: usize = I_IDX + 1;
const K_IDX_WP: usize = K_IDX + 1;
const L_IDX_WP: usize = L_IDX + 1;
const M_IDX_WP: usize = M_IDX + 1;
const N_IDX_WP: usize = N_IDX + 1;
const P_IDX_WP: usize = P_IDX + 1;
const Q_IDX_WP: usize = Q_IDX + 1;
const R_IDX_WP: usize = R_IDX + 1;
const S_IDX_WP: usize = S_IDX + 1;
const T_IDX_WP: usize = T_IDX + 1;
const V_IDX_WP: usize = V_IDX + 1;
const W_IDX_WP: usize = W_IDX + 1;
const Y_IDX_WP: usize = Y_IDX + 1;
const NOISE_IDX_WP: usize = NOISE_IDX + 1;

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


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CountArrayAN {
    proto: CountArrayProto,
}

impl CountArray for CountArrayAN {
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

impl CountArrayAN {
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
    fn chrwpidx_of_chr(chr: u8) -> usize {
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
    fn get_chridx_with_encoding_chr(chr: &mut u8) -> usize {
        match *chr {
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
                *chr = NOISE_UTF8;
                NOISE_IDX
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BwtBlock64AN {
    rank_checkpoint: [u64; CHR_COUNT],
    first_bwt_vector: u64,
    second_bwt_vector: u64,
    third_bwt_vector: u64,
    fourth_bwt_vector: u64,
    fifth_bwt_vector: u64,
}

impl BwtBlock for BwtBlock64AN {
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
            let mut fourth_bwt_vector = 0;
            let mut fifth_bwt_vector = 0;

            for c in string_chunk {
                first_bwt_vector <<= 1;
                second_bwt_vector <<= 1;
                third_bwt_vector <<= 1;
                fourth_bwt_vector <<= 1;
                fifth_bwt_vector <<= 1;
                match *c {
                    A_UTF8 => {
                        rank_checkpoint[A_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                    },
                    C_UTF8 => {
                        rank_checkpoint[C_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    D_UTF8 => {
                        rank_checkpoint[D_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    E_UTF8 => {
                        rank_checkpoint[E_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    F_UTF8 => {
                        rank_checkpoint[F_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    G_UTF8 => {
                        rank_checkpoint[G_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    H_UTF8 => {
                        rank_checkpoint[H_IDX] += 1;
                        first_bwt_vector += 1;
                    },
                    I_UTF8 => {
                        rank_checkpoint[I_IDX] += 1;
                        first_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    K_UTF8 => {
                        rank_checkpoint[K_IDX] += 1;
                        first_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    L_UTF8 => {
                        rank_checkpoint[L_IDX] += 1;
                        first_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    M_UTF8 => {
                        rank_checkpoint[M_IDX] += 1;
                        first_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    N_UTF8 => {
                        rank_checkpoint[N_IDX] += 1;
                        first_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    P_UTF8 => {
                        rank_checkpoint[P_IDX] += 1;
                        second_bwt_vector += 1;
                    },
                    Q_UTF8 => {
                        rank_checkpoint[Q_IDX] += 1;
                        second_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    R_UTF8 => {
                        rank_checkpoint[R_IDX] += 1;
                        second_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    S_UTF8 => {
                        rank_checkpoint[S_IDX] += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    T_UTF8 => {
                        rank_checkpoint[T_IDX] += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    V_UTF8 => {
                        rank_checkpoint[V_IDX] += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    W_UTF8 => {
                        rank_checkpoint[W_IDX] += 1;
                    },
                    Y_UTF8 => {
                        rank_checkpoint[Y_IDX] += 1;
                        fifth_bwt_vector += 1;
                    }
                    _ => { // NOISE
                        rank_checkpoint[NOISE_IDX] += 1;
                        fourth_bwt_vector += 1;
                    }
                }
            }
            let block = Self {
                rank_checkpoint: block_checkpoint,
                first_bwt_vector: first_bwt_vector,
                second_bwt_vector: second_bwt_vector,
                third_bwt_vector: third_bwt_vector,
                fourth_bwt_vector: fourth_bwt_vector,
                fifth_bwt_vector: fifth_bwt_vector,
            };
            
            blocks.push(block);
        });

        if last_offset == 0 {
            let last_block = Self {
                rank_checkpoint: rank_checkpoint,
                first_bwt_vector: 0,
                second_bwt_vector: 0,
                third_bwt_vector: 0,
                fourth_bwt_vector: 0,
                fifth_bwt_vector: 0,
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
        self.fourth_bwt_vector <<= offset;
        self.fifth_bwt_vector <<= offset;
    }
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_64;
        pos_bit >>= rem;

        let chridx = if self.first_bwt_vector & pos_bit == 0 {
            if self.second_bwt_vector & pos_bit == 0 {
                // 00
                if self.fourth_bwt_vector & pos_bit == 0 {
                    if self.fifth_bwt_vector & pos_bit == 0 {
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
                if self.third_bwt_vector & pos_bit == 0 {
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        // 01 10?
                        S_IDX
                    } else {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
            if self.second_bwt_vector & pos_bit == 0 {
                // 10
                if self.third_bwt_vector & pos_bit == 0 {
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        // 10 10?
                        L_IDX
                    } else {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
                if self.third_bwt_vector & pos_bit == 0 {
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        // 11 10?
                        E_IDX
                    } else {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
        let mut rank = self.rank_checkpoint[chridx];

        if rem != 0 {
            let count_bits = match chridx {
                A_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.fourth_bwt_vector &
                    !self.fifth_bwt_vector
                },
                C_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fifth_bwt_vector
                },
                D_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fourth_bwt_vector
                },
                E_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fourth_bwt_vector
                },
                F_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fifth_bwt_vector
                },
                G_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.fourth_bwt_vector &
                    self.fifth_bwt_vector
                },
                H_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    !self.fourth_bwt_vector &
                    !self.fifth_bwt_vector
                },
                I_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fifth_bwt_vector
                },
                K_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fourth_bwt_vector
                },
                L_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fourth_bwt_vector
                },
                M_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fifth_bwt_vector
                },
                N_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.fourth_bwt_vector &
                    self.fifth_bwt_vector
                },
                P_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.fourth_bwt_vector &
                    !self.fifth_bwt_vector
                },
                Q_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fifth_bwt_vector
                },
                R_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fourth_bwt_vector
                },
                S_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fourth_bwt_vector
                },
                T_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fifth_bwt_vector
                },
                V_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.fourth_bwt_vector &
                    self.fifth_bwt_vector
                },
                W_IDX => {
                    !self.first_bwt_vector &
                    !self.second_bwt_vector &
                    !self.fourth_bwt_vector &
                    !self.fifth_bwt_vector
                },
                Y_IDX => {
                    !self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.fifth_bwt_vector
                }
                _ => { // NOISE
                    !self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.fourth_bwt_vector
                }
            }  >> Self::BLOCK_SEG_LEN-rem;
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BwtBlock128AN {
    rank_checkpoint: [u64; CHR_COUNT],
    first_bwt_vector: u128,
    second_bwt_vector: u128,
    third_bwt_vector: u128,
    fourth_bwt_vector: u128,
    fifth_bwt_vector: u128,
}

impl BwtBlock for BwtBlock128AN {
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
            let mut fourth_bwt_vector = 0;
            let mut fifth_bwt_vector = 0;

            for c in string_chunk {
                first_bwt_vector <<= 1;
                second_bwt_vector <<= 1;
                third_bwt_vector <<= 1;
                fourth_bwt_vector <<= 1;
                fifth_bwt_vector <<= 1;
                match *c {
                    A_UTF8 => {
                        rank_checkpoint[A_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                    },
                    C_UTF8 => {
                        rank_checkpoint[C_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    D_UTF8 => {
                        rank_checkpoint[D_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    E_UTF8 => {
                        rank_checkpoint[E_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    F_UTF8 => {
                        rank_checkpoint[F_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    G_UTF8 => {
                        rank_checkpoint[G_IDX] += 1;
                        first_bwt_vector += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    H_UTF8 => {
                        rank_checkpoint[H_IDX] += 1;
                        first_bwt_vector += 1;
                    },
                    I_UTF8 => {
                        rank_checkpoint[I_IDX] += 1;
                        first_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    K_UTF8 => {
                        rank_checkpoint[K_IDX] += 1;
                        first_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    L_UTF8 => {
                        rank_checkpoint[L_IDX] += 1;
                        first_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    M_UTF8 => {
                        rank_checkpoint[M_IDX] += 1;
                        first_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    N_UTF8 => {
                        rank_checkpoint[N_IDX] += 1;
                        first_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    P_UTF8 => {
                        rank_checkpoint[P_IDX] += 1;
                        second_bwt_vector += 1;
                    },
                    Q_UTF8 => {
                        rank_checkpoint[Q_IDX] += 1;
                        second_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    R_UTF8 => {
                        rank_checkpoint[R_IDX] += 1;
                        second_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    S_UTF8 => {
                        rank_checkpoint[S_IDX] += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    T_UTF8 => {
                        rank_checkpoint[T_IDX] += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                    },
                    V_UTF8 => {
                        rank_checkpoint[V_IDX] += 1;
                        second_bwt_vector += 1;
                        third_bwt_vector += 1;
                        fourth_bwt_vector += 1;
                        fifth_bwt_vector += 1;
                    },
                    W_UTF8 => {
                        rank_checkpoint[W_IDX] += 1;
                    },
                    Y_UTF8 => {
                        rank_checkpoint[Y_IDX] += 1;
                        fifth_bwt_vector += 1;
                    }
                    _ => { // NOISE
                        rank_checkpoint[NOISE_IDX] += 1;
                        fourth_bwt_vector += 1;
                    }
                }
            }
            let block = Self {
                rank_checkpoint: block_checkpoint,
                first_bwt_vector: first_bwt_vector,
                second_bwt_vector: second_bwt_vector,
                third_bwt_vector: third_bwt_vector,
                fourth_bwt_vector: fourth_bwt_vector,
                fifth_bwt_vector: fifth_bwt_vector,
            };
            
            blocks.push(block);
        });

        if last_offset == 0 {
            let last_block = Self {
                rank_checkpoint: rank_checkpoint,
                first_bwt_vector: 0,
                second_bwt_vector: 0,
                third_bwt_vector: 0,
                fourth_bwt_vector: 0,
                fifth_bwt_vector: 0,
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
        self.fourth_bwt_vector <<= offset;
        self.fifth_bwt_vector <<= offset;
    }
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64) {
        let mut pos_bit = POS_BIT_128;
        pos_bit >>= rem;

        let chridx = if self.first_bwt_vector & pos_bit == 0 {
            if self.second_bwt_vector & pos_bit == 0 {
                // 00
                if self.fourth_bwt_vector & pos_bit == 0 {
                    if self.fifth_bwt_vector & pos_bit == 0 {
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
                if self.third_bwt_vector & pos_bit == 0 {
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        // 01 10?
                        S_IDX
                    } else {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
            if self.second_bwt_vector & pos_bit == 0 {
                // 10
                if self.third_bwt_vector & pos_bit == 0 {
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        // 10 10?
                        L_IDX
                    } else {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
                if self.third_bwt_vector & pos_bit == 0 {
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
                    if self.fourth_bwt_vector & pos_bit == 0 {
                        // 11 10?
                        E_IDX
                    } else {
                        if self.fifth_bwt_vector & pos_bit == 0 {
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
        let mut rank = self.rank_checkpoint[chridx];

        if rem != 0 {
            let count_bits = match chridx {
                A_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.fourth_bwt_vector &
                    !self.fifth_bwt_vector
                },
                C_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fifth_bwt_vector
                },
                D_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fourth_bwt_vector
                },
                E_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fourth_bwt_vector
                },
                F_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fifth_bwt_vector
                },
                G_IDX => {
                    self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.fourth_bwt_vector &
                    self.fifth_bwt_vector
                },
                H_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    !self.fourth_bwt_vector &
                    !self.fifth_bwt_vector
                },
                I_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fifth_bwt_vector
                },
                K_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fourth_bwt_vector
                },
                L_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fourth_bwt_vector
                },
                M_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fifth_bwt_vector
                },
                N_IDX => {
                    self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.fourth_bwt_vector &
                    self.fifth_bwt_vector
                },
                P_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.fourth_bwt_vector &
                    !self.fifth_bwt_vector
                },
                Q_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fifth_bwt_vector
                },
                R_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    !self.third_bwt_vector &
                    self.fourth_bwt_vector
                },
                S_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fourth_bwt_vector
                },
                T_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.third_bwt_vector &
                    !self.fifth_bwt_vector
                },
                V_IDX => {
                    !self.first_bwt_vector &
                    self.second_bwt_vector &
                    self.fourth_bwt_vector &
                    self.fifth_bwt_vector
                },
                W_IDX => {
                    !self.first_bwt_vector &
                    !self.second_bwt_vector &
                    !self.fourth_bwt_vector &
                    !self.fifth_bwt_vector
                },
                Y_IDX => {
                    !self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.fifth_bwt_vector
                }
                _ => { // NOISE
                    !self.first_bwt_vector &
                    !self.second_bwt_vector &
                    self.fourth_bwt_vector
                }
            }  >> Self::BLOCK_SEG_LEN-rem;
            rank += count_bits.count_ones() as u64;
        };

        rank
    }
}