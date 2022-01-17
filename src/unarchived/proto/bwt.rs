use super::{Serialize, Deserialize};
use super::{Text};

use super::Bwt;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BwtProto<W: BwtBlock> {
    primary_index: u64,
    blocks: Vec<W>,
}

impl<W: BwtBlock> Bwt for BwtProto<W> {
    fn new(bwt_text: Text, pidx: u64) -> Self {
        let blocks: Vec<W> = BwtBlock::new_with_bwt_text(bwt_text);

        Self {
            primary_index: pidx,
            blocks: blocks,
        }
    }
    fn get_pre_chridx_and_rank_of_pos(&self, mut pos: u64) -> Option<(usize, u64)> {
        if pos == self.primary_index - 1 {
            return None;
        } else if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / W::BLOCK_SEG_LEN;
        let rem = pos % W::BLOCK_SEG_LEN;

        let (chridx, rank) = self.blocks[quot as usize].get_chridx_and_rank_of_rem(rem);
        Some((chridx, rank))
    }
    fn get_next_rank_of_pos_and_chridx(&self, mut pos: u64, chridx: usize) -> u64 {
        if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / W::BLOCK_SEG_LEN;
        let rem = pos % W::BLOCK_SEG_LEN;

        self.blocks[quot as usize].get_rank_of_chridx_and_rem(chridx, rem)
    }
}

pub trait BwtBlock {
    const BLOCK_SEG_LEN: u64;
    
    fn new_with_bwt_text(bwt_text: Text) -> Vec<Self> where Self: Sized;
    fn add_offset(&mut self, offset: usize);
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64);
    fn get_rank_of_chridx_and_rem(&self, chridx: usize, rem: u64) -> u64;
}