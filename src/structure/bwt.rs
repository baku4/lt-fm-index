use super::{Text, Serialize, Deserialize};

const BLOCK_SEG_LEN: u64 = 64;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Bwt<B: BwtBlockInterface> {
    primary_index: u64,
    blocks: Vec<B>,
    bitcount_lookup_table: BitcountLookupTable,
}

impl<B: BwtBlockInterface> Bwt<B> {
    pub fn new(text: Text, pidx: u64) {

    }

    pub fn get_pre_chridx_and_rank_of_pos(&self, mut pos: u64) -> Option<(usize, u64)> {
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
    pub fn get_next_rank_of_pos_and_chridx(&self, mut pos: u64, chridx: usize) -> u64 {
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

pub trait BwtBlockInterface {
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64);
    fn get_rank_of_chridx(&self, chridx: usize) -> u64;
    fn get_rank_of_chridx_and_rem(&self, chridx: usize, rem: u64) -> u64;
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum BitcountLookupTable {
    Bit8CountTable,
    Bit16CountTable,
}

struct BwtBlock {
    
}