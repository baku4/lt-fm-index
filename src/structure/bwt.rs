use super::{
    Result, error_msg,
    Archive, Serialize, Deserialize,
    Text, Pattern,
};
use super::{
    BwtConstructor, BwtInterface,
};


// Bwt Structure

#[derive(Archive, Serialize, Deserialize)]
#[archive(archived = "Bwt")]
pub struct BwtPreBuild<W: BwtBlockConstructor> {
    primary_index: u64,
    blocks: Vec<W>,
}


// Bwt Implementations

impl<W> BwtConstructor for BwtPreBuild<W> where
    W: BwtBlockConstructor,
{
    fn new(bwt_text: Text, pidx: u64) -> Self {
        let blocks: Vec<W> = W::new_with_bwt_text(bwt_text);

        Self {
            primary_index: pidx,
            blocks: blocks,
        }
    }
}

impl<W> BwtInterface for Bwt<W> where
    W: BwtBlockConstructor + Archive,
    W::Archived: BwtBlockInterface,
{
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


// BwtBlock Requirements

pub trait BwtBlockConstructor {
    const BLOCK_SEG_LEN: u64;

    fn new_with_bwt_text(bwt_text: Text) -> Vec<Self> where Self: Sized;
}

pub trait BwtBlockInterface {
    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64);
    fn get_rank_of_chridx_and_rem(&self, chridx: usize, rem: u64) -> u64;
}
