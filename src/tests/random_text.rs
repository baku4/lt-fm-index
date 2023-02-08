use std::ops::Range;
use rand::{Rng, seq::SliceRandom};

pub const NO_STEMS: [u8; 3] = [b'A', b'C', b'G'];
pub const NN_STEMS: [u8; 4] = [b'A', b'C', b'G', b'T'];
pub const AO_STEMS: [u8; 19] = [b'A', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'V', b'W'];
pub const AN_STEMS: [u8; 20] = [b'A', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'K', b'L', b'M', b'N', b'P', b'Q', b'R', b'S', b'T', b'V', b'W', b'Y'];
const TEXT_LENGTH_RANGE: Range<usize> = 50..200;
const ASCII_RANGE: Range<u8> = 33..127;

pub fn gen_rand_text(chr_stem: &[u8], text_len_range: Range<usize>) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    // chr list
    let mut chr_list = chr_stem.to_vec();
    loop {
        let addt: u8 = rng.gen_range(ASCII_RANGE);
        if !chr_list.contains(&addt) {
            chr_list.push(addt);
            break;
        }
    }
    // make text
    let chr_count = chr_list.len();
    let mut text = chr_list.clone();
    let text_len: usize = rng.gen_range(text_len_range);
    if text_len > chr_list.len() {
        let remain = text_len - chr_count;
        for _ in 0..remain {
            let chr = chr_list[rng.gen_range(0..chr_count)];
            text.push(chr);
        }
    }
    // shuffle
    text.shuffle(&mut rng);
    
    text
}
pub fn gen_rand_text_with_default_len(chr_stem: &[u8]) ->  Vec<u8> {
    gen_rand_text(chr_stem, TEXT_LENGTH_RANGE)
}
pub fn rand_text_of_no() -> Vec<u8> {
    gen_rand_text_with_default_len(&NO_STEMS)
}
pub fn rand_text_of_nn() -> Vec<u8> {
    gen_rand_text_with_default_len(&NN_STEMS)
}
pub fn rand_text_of_ao() -> Vec<u8> {
    gen_rand_text_with_default_len(&AO_STEMS)
}
pub fn rand_text_of_an() -> Vec<u8> {
    gen_rand_text_with_default_len(&AN_STEMS)
}

pub fn rand_pattern_of_text(text: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let start = rng.gen_range(0..text.len() - 1);
    let end = rng.gen_range(start+1..text.len());
    text[start..end].to_vec()
}
pub fn rand_pattern_of_length(text: &[u8], length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let last_idx = text.len() - length;
    let start = rng.gen_range(0..last_idx);
    let end = start + length;
    text[start..end].to_vec()
}