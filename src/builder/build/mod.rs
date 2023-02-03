use crate::core::{
    Text,
};
use super::{
    LtFmIndex,
    LtFmIndexBuilder,
    BwtBlockSize,
    TextType,
    BuildError,
};

impl LtFmIndexBuilder {
    pub fn build(self, text: Text) -> Result<LtFmIndex, BuildError> {
        let text_type = match self.text_type {
            Some(v) => v,
            None => {
                let text_type = TextType::new_inferred(&text);
                text_type.unwrap()
            }
        };
        let lookup_table_kmer_size = match self.lookup_table_kmer_size {
            Some(v) => v,
            None => text_type.recommend_kmer_size(),
        };

        let lt_fm_index = LtFmIndex::new(
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
impl TextType {
    // The next case include all characters of previous case
    //  1. ACG*
    //  2. ACGT*
    //  3. ACDEFGHIKLMNPQRSTVW*
    //  4. ACDEFGHIKLMNPQRSTVWY*
    //
    // Use bitwise-flag
    // 0_0000_0000_0000_0000
    // D_EFHI_KLMN_PQRS_TVWY
    const D_FLAG: u32 = 0b1_0000_0000_0000_0000;
    const E_FLAG: u32 = 0b0_1000_0000_0000_0000;
    const F_FLAG: u32 = 0b0_0100_0000_0000_0000;
    const H_FLAG: u32 = 0b0_0010_0000_0000_0000;
    const I_FLAG: u32 = 0b0_0001_0000_0000_0000;
    const K_FLAG: u32 = 0b0_0000_1000_0000_0000;
    const L_FLAG: u32 = 0b0_0000_0100_0000_0000;
    const M_FLAG: u32 = 0b0_0000_0010_0000_0000;
    const N_FLAG: u32 = 0b0_0000_0001_0000_0000;
    const P_FLAG: u32 = 0b0_0000_0000_1000_0000;
    const Q_FLAG: u32 = 0b0_0000_0000_0100_0000;
    const R_FLAG: u32 = 0b0_0000_0000_0010_0000;
    const S_FLAG: u32 = 0b0_0000_0000_0001_0000;
    const T_FLAG: u32 = 0b0_0000_0000_0000_1000;
    const V_FLAG: u32 = 0b0_0000_0000_0000_0100;
    const W_FLAG: u32 = 0b0_0000_0000_0000_0010;
    const Y_FLAG: u32 = 0b0_0000_0000_0000_0001;

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
                // FIXME: Write logic
                panic!()
            },
            ControlFlow::Break(chr) => {
                // FIXME: Write logic
                panic!()
            },
        }
    }
    pub fn recommend_kmer_size(&self) -> usize {
        match self {
            Self::NucleotideOnly => 8, // About 64 Kb for kmer count array
            Self::NucleotideWithNoise => 7, // About 76 Kb for kmer count array
            Self::AminoAcidOnly => 4, // About 156 Kb for kmer count array
            Self::AminoAcidWithNoise => 4, // About 190 Kb for kmer count array
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn infer_text_type() {
        // TODO: Write
    }
}