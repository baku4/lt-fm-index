use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
};
use super::{
    TextEncoder,
    BwtBlockConstructor, BwtBlockInterface,
};

const CHR_COUNT: usize = 4;

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

#[derive(Archive, Serialize, Deserialize)]
#[archive(archived = "BwtBlockNO")]
pub struct BwtBlockNOPreBuild<T> {
    rank_checkpoint: [u64; 4],
    first_bwt_vector: T,
    second_bwt_vector: T,
}
