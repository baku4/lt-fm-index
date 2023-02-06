use crate::core::{
    Text,
};
use super::{
    LtFmIndexDep,
    LtFmIndexBuilder,
    TextTypeDep,
    BuildError,
};

impl LtFmIndexBuilder {
    pub fn build(self, text: Text) -> Result<LtFmIndexDep, BuildError> {
        let text_type = match self.text_type {
            Some(v) => v,
            None => {
                let text_type = TextTypeDep::new_inferred(&text);
                text_type.unwrap()
            }
        };
        let lookup_table_kmer_size = match self.lookup_table_kmer_size {
            Some(v) => v,
            None => text_type.recommend_kmer_size(),
        };

        let lt_fm_index = LtFmIndexDep::new(
            text,
            self.suffix_array_sampling_ratio,
            lookup_table_kmer_size,
            text_type,
            self.bwt_block_size,
        );

        Ok(lt_fm_index)
    }
}

use std::ops::ControlFlow;
impl TextTypeDep {
    // The next case include all characters of previous case
    //  1. ACG*
    //  2. ACGT*
    //  3. ACDEFGHIKLMNPQRSTVW*
    //  4. ACDEFGHIKLMNPQRSTVWY*
    //
    // Use bitwise-flag
    // 0_0000_0000_0000_0000
    // D_EFHI_KLMN_PQRS_TVWY
    const D_FLAG: u32 = 1 << 16;
    const E_FLAG: u32 = 1 << 15;
    const F_FLAG: u32 = 1 << 14;
    const H_FLAG: u32 = 1 << 13;
    const I_FLAG: u32 = 1 << 12;
    const K_FLAG: u32 = 1 << 11;
    const L_FLAG: u32 = 1 << 10;
    const M_FLAG: u32 = 1 << 9;
    const N_FLAG: u32 = 1 << 8;
    const P_FLAG: u32 = 1 << 7;
    const Q_FLAG: u32 = 1 << 6;
    const R_FLAG: u32 = 1 << 5;
    const S_FLAG: u32 = 1 << 4;
    const T_FLAG: u32 = 1 << 3;
    const V_FLAG: u32 = 1 << 2;
    const W_FLAG: u32 = 1 << 1;
    const Y_FLAG: u32 = 1;

    pub fn new_inferred(text: &Text) -> Result<Self, BuildError> {
        let mut bit_flag: u32 = 0;
        let mut addt_chr: Option<u8> = None;
        let errored_chr = text.iter().try_for_each(|&chr| {
            match chr {
                b'A' | b'C' | b'G' => { ControlFlow::Continue(()) },
                b'D' => { bit_flag |= Self::D_FLAG; ControlFlow::Continue(()) },
                b'E' => { bit_flag |= Self::E_FLAG; ControlFlow::Continue(()) },
                b'F' => { bit_flag |= Self::F_FLAG; ControlFlow::Continue(()) },
                b'H' => { bit_flag |= Self::H_FLAG; ControlFlow::Continue(()) },
                b'I' => { bit_flag |= Self::I_FLAG; ControlFlow::Continue(()) },
                b'K' => { bit_flag |= Self::K_FLAG; ControlFlow::Continue(()) },
                b'L' => { bit_flag |= Self::L_FLAG; ControlFlow::Continue(()) },
                b'M' => { bit_flag |= Self::M_FLAG; ControlFlow::Continue(()) },
                b'N' => { bit_flag |= Self::N_FLAG; ControlFlow::Continue(()) },
                b'P' => { bit_flag |= Self::P_FLAG; ControlFlow::Continue(()) },
                b'Q' => { bit_flag |= Self::Q_FLAG; ControlFlow::Continue(()) },
                b'R' => { bit_flag |= Self::R_FLAG; ControlFlow::Continue(()) },
                b'S' => { bit_flag |= Self::S_FLAG; ControlFlow::Continue(()) },
                b'T' => { bit_flag |= Self::T_FLAG; ControlFlow::Continue(()) },
                b'V' => { bit_flag |= Self::V_FLAG; ControlFlow::Continue(()) },
                b'W' => { bit_flag |= Self::W_FLAG; ControlFlow::Continue(()) },
                b'Y' => { bit_flag |= Self::Y_FLAG; ControlFlow::Continue(()) },
                _ => {
                    match addt_chr {
                        Some(v) => {
                            if v != chr {
                                ControlFlow::Break(chr)
                            } else {
                                ControlFlow::Continue(())
                            }
                        },
                        None => {
                            addt_chr = Some(chr);
                            ControlFlow::Continue(())
                        },
                    }
                },
            }
        });
        
        match errored_chr {
            ControlFlow::Continue(_) => {
                let addt_chr_count = if addt_chr == None { 0 } else { 1 };
                let case_1_noise_cand_count = bit_flag.count_ones() + addt_chr_count;
                if case_1_noise_cand_count <= 1 {
                    return Ok(TextTypeDep::NucleotideOnly)
                }
                let case_2_noise_cand_count = (bit_flag & 0b1_1111_1111_1111_0111).count_ones() + addt_chr_count;
                if case_2_noise_cand_count <= 1 {
                    return Ok(TextTypeDep::NucleotideWithNoise)
                }
                let case_3_noise_cand_count = (bit_flag & 0b0_0000_0000_0000_0001).count_ones() + addt_chr_count;
                if case_3_noise_cand_count <= 1 {
                    return Ok(TextTypeDep::AminoAcidOnly)
                }
                Ok(TextTypeDep::AminoAcidWithNoise) // Case 4
            },
            ControlFlow::Break(chr) => {
                Err(BuildError::TextTypeError(char::from(addt_chr.unwrap()), char::from(chr)))
            },
        }
    }
    pub fn recommend_kmer_size(&self) -> usize {
        match self {
            // The size of kmer count table is about:
            Self::NucleotideOnly => 7, // 610 KiB (8*5^7)
            Self::NucleotideWithNoise => 6, // 364 KiB (8*6^6)
            Self::AminoAcidOnly => 4, // 1.48 MiB (8*21^4)
            Self::AminoAcidWithNoise => 4, // 1.79 MiB (8*22^4)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        TextTypeDep,
    };
    use crate::tests::random_text::*;

    #[test]
    fn infer_text_type() {
        let n = 100;
        for _ in 0..n {
            let no_text = rand_text_of_no();
            let type_ = TextTypeDep::new_inferred(&no_text).unwrap();
            assert_eq!(type_, TextTypeDep::NucleotideOnly);

            let nn_text = rand_text_of_nn();
            let type_ = TextTypeDep::new_inferred(&nn_text).unwrap();
            assert_eq!(type_, TextTypeDep::NucleotideWithNoise);

            let ao_text = rand_text_of_ao();
            let type_ = TextTypeDep::new_inferred(&ao_text).unwrap();
            assert_eq!(type_, TextTypeDep::AminoAcidOnly);

            let an_text = rand_text_of_an();
            let type_ = TextTypeDep::new_inferred(&an_text).unwrap();
            assert_eq!(type_, TextTypeDep::AminoAcidWithNoise)
        }
    }
}