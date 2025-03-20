use crate::core::Position;

pub mod blocks;

// Burrows-Wheeler Matrix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bwm<P: Position, B: Block<P>> {
    primary_index: P,
    chr_count: u32,
    rank_checkpoints: Vec<P>,
    blocks: Vec<B>,
}
/**
Type of the block of compressed Burrow-Wheeler transformed text.
The implementations are in [blocks] module.
*/
pub trait Block<P: Position>: Sized + bytemuck::Pod {
    const BLOCK_LEN: u32;
    const MAX_CHR: u32;
    // Build
    fn vectorize(bwt_text: &[u8], rank_pre_counts: &mut Vec<P>) -> Self;
    fn empty() -> Self;
    fn shift_last_offset(&mut self, offset: u32);
    // Locate
    fn get_remain_count_of(&self, rem: u32, chridx: u8) -> u32;
    fn get_chridx_of(&self, rem: u32) -> u8;
}

// Bwm Implementations
impl<P: Position, B: Block<P>> Bwm<P, B> {
    // Build
    #[inline]
    pub fn new(bwt_text: Vec<u8>, pidx: P, chr_count: u32) -> Self {
        let block_len = B::BLOCK_LEN;
        let mut chunk_count = bwt_text.len() as u32 / block_len.as_u32();
        let rem = bwt_text.len() as u32 % block_len.as_u32();
        
        let last_offset = if rem == 0 {
            chunk_count += 1;
            rem
        } else {
            block_len.as_u32() - rem
        };

        let mut rank_checkpoints = Vec::with_capacity((chunk_count * chr_count) as usize);
        let mut rank_pre_counts = vec![P::ZERO; chr_count as usize];
        let mut blocks: Vec<B> = Vec::with_capacity(chunk_count as usize);

        bwt_text.chunks(block_len.as_usize()).for_each(|text_chunk| {
            rank_checkpoints.extend_from_slice(&rank_pre_counts);
            let block = B::vectorize(text_chunk, &mut rank_pre_counts);
            blocks.push(block);
        });

        if last_offset == 0 {
            rank_checkpoints.extend_from_slice(&rank_pre_counts);
            blocks.push(B::empty());
        } else {
            let last_block = blocks.last_mut().unwrap();
            last_block.shift_last_offset(last_offset);
        }

        Self {
            primary_index: pidx,
            chr_count,
            rank_checkpoints,
            blocks: blocks,
        }
    }
    // Locate
    #[inline]
    pub fn get_next_rank(&self, mut pos: P, chridx: u8) -> P {
        if pos < self.primary_index {
            pos += P::ONE;
        }
        let quot = pos.as_u32() / B::BLOCK_LEN;
        let rem = pos.as_u32() % B::BLOCK_LEN;

        let rank_idx = quot.as_u32() * self.chr_count + chridx as u32;
        let rank_precount = self.rank_checkpoints[rank_idx as usize];
        if rem == 0 {
            rank_precount
        } else {
            let rem_count = self.blocks[quot.as_usize()].get_remain_count_of(rem, chridx);
            rank_precount + P::from_u32(rem_count)
        }
    }
    #[inline]
    pub fn get_pre_rank_and_chridx(&self, mut pos: P) -> Option<(P, u8)> {
        if pos == self.primary_index - P::ONE {
            return None;
        } else if pos < self.primary_index {
            pos += P::ONE;
        }
        let quot = pos.as_u32() / B::BLOCK_LEN;
        let rem = pos.as_u32() % B::BLOCK_LEN;
        
        let block = &self.blocks[quot.as_usize()];
        let chridx = block.get_chridx_of(rem);

        let rank_idx = quot.as_u32() * self.chr_count + chridx as u32;
        let rank_precount = self.rank_checkpoints[rank_idx as usize];
        if rem == 0 {
            Some((rank_precount, chridx))
        } else {
            let rem_count = block.get_remain_count_of(rem, chridx);
            Some((rank_precount + P::from_u32(rem_count), chridx))
        }
    }

    pub fn chr_count(&self) -> u32 {
        self.chr_count
    }
}

mod serialize;