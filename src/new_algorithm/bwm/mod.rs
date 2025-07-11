use num_integer::div_rem;
use zerocopy::IntoBytes;

use crate::core::Position;

pub mod blocks;

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes, zerocopy::Immutable, zerocopy::KnownLayout)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BwmHeader {
    // Given
    pub chr_with_sentinel_count: u32,
    _padding: u32,
    // Derivatives
    pub rank_checkpoints_len: u64,
    pub blocks_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BwmView<'a, P: Position, B: Block> {
    // From header
    chr_with_sentinel_count: P,
    // From blob
    sentinel_chr_index: P,
    rank_checkpoints: &'a [P],
    blocks: &'a [B],
}

pub trait Block: zerocopy::FromBytes + zerocopy::IntoBytes + zerocopy::Immutable {
    const BLOCK_LEN: u32; // Length of block
    const MAX_CHR: u32; // Maximum character that can be indexed by the block

    // Build
    fn vectorize<P: Position>(bwt_text: &[u8], rank_pre_counts: &mut Vec<P>) -> Self;
    fn shift_last_offset(&mut self, offset: u32);
    // Locate
    fn get_remain_count_of(&self, rem: u32, chridx: u8) -> u32;
    fn get_chridx_of(&self, rem: u32) -> u8;
}

// ================================================
// Build
// ================================================
impl BwmHeader {
    #[inline]
    pub fn new<P: Position, B: Block>(
        text_len: u64,
        chr_with_sentinel_count: u32, // chr_count + 1 (sentinel)
    ) -> Self {
        let block_len = B::BLOCK_LEN;

        let (mut block_count, rem) = div_rem(text_len, block_len as u64);

        if rem == 0 {
            // If the last block is full, add one more chunk to add rank_pre_counts
            block_count += 1;
        }

        let rank_checkpoints_len = (block_count as u64) * (chr_with_sentinel_count as u64);
        let blocks_len = block_count as u64;

        Self {
            chr_with_sentinel_count,
            _padding: 0,
            rank_checkpoints_len,
            blocks_len,
        }
    }
    pub fn calculate_body_size<P: Position, B: Block>(
        &self,
    ) -> usize {
        let sentinel_chr_index_size = std::mem::size_of::<P>();
        let rank_checkpoints_size = self.rank_checkpoints_len as usize * std::mem::size_of::<P>();
        let blocks_size = self.blocks_len as usize * std::mem::size_of::<B>();

        sentinel_chr_index_size + rank_checkpoints_size + blocks_size
    }
    pub fn write_to_blob<P: Position, B: Block>(
        &self,
        bwt_text: Vec<u8>, // burrow-wheeler transformed text
        sentinel_chr_index: u32, // Sentinel character index in bwt_text
        blob: &mut [u8],
    ) {
        let last_offset = {
            let rem = bwt_text.len() % B::BLOCK_LEN as usize;
            if rem == 0 {
                0
            } else {
                B::BLOCK_LEN - (rem as u32)
            }
        };

        // Write sentinel_chr_index
        let sentinel_chr_index_size = std::mem::size_of::<P>();
        let sentinel_chr_index_blob = &mut blob[..sentinel_chr_index_size];
        sentinel_chr_index_blob.copy_from_slice(&sentinel_chr_index.as_bytes());

        // Divide blob into rank_checkpoints and blocks
        let rank_checkpoints_size = self.rank_checkpoints_len as usize * std::mem::size_of::<P>();
        let (rank_checkpoints_blob, blocks_blob) = {
            let (left, right) = blob[sentinel_chr_index_size..].split_at_mut(rank_checkpoints_size);
            let left: &mut [P] = zerocopy::FromBytes::mut_from_bytes(left).unwrap();
            let right: &mut [B] = zerocopy::FromBytes::mut_from_bytes(right).unwrap();
            (left, right)
        };

        let mut rank_pre_counts = vec![P::ZERO; self.chr_with_sentinel_count as usize];

        bwt_text.chunks(B::BLOCK_LEN as usize).enumerate().for_each(|(block_idx, text_chunk)| {
            rank_checkpoints_blob.copy_from_slice(&rank_pre_counts);
            let block = B::vectorize(text_chunk, &mut rank_pre_counts);
            blocks_blob[block_idx] = block;
        });

        if last_offset == 0 {
            rank_checkpoints_blob.copy_from_slice(&rank_pre_counts);
            blocks_blob[self.blocks_len as usize - 1].as_mut_bytes().fill(0);
        } else {
            let last_block = blocks_blob.last_mut().unwrap();
            last_block.shift_last_offset(last_offset);
        }
    }
}

// ================================================
// Load
// ================================================
impl BwmHeader {
    pub fn load<'a, P: Position, B: Block>(&self, body_blob: &'a [u8]) -> BwmView<'a, P, B> {
        let chr_with_sentinel_count = P::from_u32(self.chr_with_sentinel_count);

        // Sentinel chr index
        let mut body_start_index = 0;
        let mut body_end_index = std::mem::size_of::<P>();
        let sentinel_chr_index_blob = &body_blob[body_start_index..body_end_index];
        let sentinel_chr_index = zerocopy::FromBytes::read_from_bytes(sentinel_chr_index_blob).unwrap();

        // Rank checkpoints
        body_start_index = body_end_index;
        body_end_index += self.rank_checkpoints_len as usize * std::mem::size_of::<P>();
        let rank_checkpoints_blob = &body_blob[body_start_index..body_end_index];
        let rank_checkpoints: &[P] = zerocopy::FromBytes::ref_from_bytes(rank_checkpoints_blob).unwrap();
        // Blocks
        body_start_index = body_end_index;
        body_end_index += self.blocks_len as usize * std::mem::size_of::<B>();
        let blocks_blob = &body_blob[body_start_index..body_end_index];
        let blocks: &[B] = zerocopy::FromBytes::ref_from_bytes(blocks_blob).unwrap();

        BwmView {
            chr_with_sentinel_count,
            sentinel_chr_index,
            rank_checkpoints,
            blocks,
        }
    }
}

// ================================================
// Locate
// ================================================
impl<'a, P: Position, B: Block> BwmView<'a, P, B> {
    #[inline]
    pub fn get_next_rank(
        &self,
        mut pos: P,
        chridx: u8,
    ) -> P {
        if pos < self.sentinel_chr_index {
            pos += P::ONE;
        }
        let (quot, rem) = pos.div_rem_with_u32(B::BLOCK_LEN);

        let rank_idx = quot.as_usize() * self.chr_with_sentinel_count.as_usize() + chridx as usize;
        let rank_precount = self.rank_checkpoints[rank_idx];
        if rem == 0 {
            rank_precount
        } else {
            let rem_count = self.blocks[quot.as_usize()].get_remain_count_of(rem, chridx);
            rank_precount + P::from_u32(rem_count)
        }
    }
    #[inline]
    pub fn get_pre_rank_and_chridx(&self, mut pos: P) -> Option<(P, u8)> {
        if pos == self.sentinel_chr_index - P::ONE {
            return None;
        } else if pos < self.sentinel_chr_index {
            pos += P::ONE;
        }
        let (quot, rem) = pos.div_rem_with_u32(B::BLOCK_LEN);
        
        let block = &self.blocks[quot.as_usize()];
        let chridx = block.get_chridx_of(rem);

        let rank_idx = quot.as_usize() * self.chr_with_sentinel_count.as_usize() + chridx as usize;
        let rank_precount = self.rank_checkpoints[rank_idx];
        if rem == 0 {
            Some((rank_precount, chridx))
        } else {
            let rem_count = block.get_remain_count_of(rem, chridx);
            Some((rank_precount + P::from_u32(rem_count), chridx))
        }
    }
}
