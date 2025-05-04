use std::ops::Range;
use rand::{Rng, seq::SliceRandom};

const ASCII_RANGE: Range<u8> = 33..127;

pub fn gen_rand_chr_list(chr_count: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    let mut chr_list = Vec::with_capacity(chr_count);
    while chr_list.len() < chr_count {
        let chr = rng.random_range(ASCII_RANGE);
        if !chr_list.contains(&chr) {
            chr_list.push(chr);
        }
    }
    chr_list
}
// Return the length of min(chr_list.len(), min_len). At least one chr in chr_list is included in text.
pub fn gen_rand_text(chr_list: &[u8], min_len: usize, max_len: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    let text_len = rng.random_range(min_len..max_len+1);
    let chr_count = chr_list.len();
    let mut text = chr_list.to_vec();
    while text.len() < text_len {
        let chr = chr_list[rng.random_range(0..chr_count)];
        text.push(chr);
    }
    text.shuffle(&mut rng);
    text
}
pub fn gen_rand_pattern(text: &[u8], min_len: usize, max_len: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    let pattern_len = rng.random_range(min_len..max_len+1);
    let last_idx = text.len() - pattern_len;
    let start = rng.random_range(0..last_idx);
    let end = start + pattern_len;
    text[start..end].to_vec()
}
