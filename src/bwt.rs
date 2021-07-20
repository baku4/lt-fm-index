// #[cfg(feture = )]
struct BwtBlock {
    rank_checkpoint: [u64; 4],
    compressed_bwt: [u8; 4],
}

pub struct Bwt {
    blocks: Vec<BwtBlock>,
    base_lookup_table: Vec<u32>,
}

impl Bwt {
    #[inline]
    pub fn new(segment_size: usize) {

    }
    #[inline]
    pub fn lf_map_with_range(&self, pos_range: (u64, u64), c: u8) -> (u64, u64) {
        (0, 0)
    }
    #[inline]
    pub fn lf_map_with_pos(&self, pos: u64) -> u64 {
        0
    }
}