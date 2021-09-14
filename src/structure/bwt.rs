use std::array;

use super::BwtInterface;
use super::{Serialize, Deserialize};

const BLOCK_SEG_LEN: u64 = 64;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Bwt<B: BwtBlock> {
    primary_index: u64,
    blocks: Vec<B>,
    bitcount_lookup_table: BitcountLookupTable,
}

impl<B: BwtBlock> BwtInterface for Bwt<B> {
    fn get_pre_chridx_and_rank_of_pos(&self, mut pos: u64) -> Option<(usize, u64)> {
        if pos == self.primary_index - 1 {
            return None;
        } else if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / BLOCK_SEG_LEN;
        let rem = pos % BLOCK_SEG_LEN;

        let (chridx, rank) = self.blocks[quot as usize].get_chridx_and_rank_of_rem(rem);
        Some((chridx, rank))
    }
    fn get_next_rank_of_pos_and_chridx(&self, mut pos: u64, chridx: usize) -> u64 {
        if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / BLOCK_SEG_LEN;
        let rem = pos % BLOCK_SEG_LEN;
        if rem == 0 {
            self.blocks[quot as usize].get_rank_of_chridx(chridx)
        } else {
            self.blocks[quot as usize].get_rank_of_chridx_and_rem(chridx, rem)
        }
    }
}

trait BwtBlock {
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64);
    fn get_rank_of_chridx(&self, chridx: usize) -> u64;
    fn get_rank_of_chridx_and_rem(&self, chridx: usize, rem: u64) -> u64;
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum BitcountLookupTable {
    Bit8CountTable,
    Bit16CountTable,
}