use super::{
    TextEncoder,
    BwtBlock,
    Block,
};
use super::*;

macro_rules! TextEncoder {
    () => {
        
    };
}

struct Four64 {
    
}

Block!(Four64Block, 4, u64);
impl TextEncoder for Four64 {
    type BwtBlockType = Four64Block;

    fn chr_count(&self) -> usize {
        4
    }
    fn wildcard(&self) -> u8 {
        b'_'
    }
    fn chr_idx_table(&self) -> [u8; 256] {
        let mut table = [3; 256];
        table[b'A' as usize] = 0;
        table[b'C' as usize] = 1;
        table[b'G' as usize] = 2;
        table
    }
}
