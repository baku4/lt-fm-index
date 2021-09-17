pub mod count_array;
pub mod bwt;

pub use count_array::CountArrayProto;

use crate::structure::CountArray;

use crate::{Serialize, Deserialize};
use crate::{Text, Pattern};

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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct CountArrayTest {
    proto: CountArrayProto,
}

impl CountArray for CountArrayTest {
    fn new_and_encode_text(text: &mut Text, kmer_size: Option<usize>) -> Self {
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
        self.proto.get_initial_pos_range_and_idx_of_pattern(pattern, Self::chridx_of_chr, Self::chrwpidx_of_chr)
    }
}

impl CountArrayTest {
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