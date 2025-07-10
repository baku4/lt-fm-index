use num_integer::div_rem;

use crate::core::Position;

pub mod blocks;

#[repr(C)]
#[derive(zerocopy::FromBytes, zerocopy::IntoBytes)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BwmHeader {
    // Given
    pub sentinel_chr_index: u64,
    pub chr_with_sentinel_count: u32,
    _padding: u32,
    // Derivatives
    pub rank_checkpoints_len: u64,
    pub blocks_len: u64,
}

pub struct BwmView<'a, P: Position, B: Block> {
    // From header
    sentinel_chr_index: P,
    chr_with_sentinel_count: P,
    // From blob
    rank_checkpoints: &'a [P],
    blocks: &'a [B],
}

trait Block: zerocopy::FromBytes + zerocopy::IntoBytes {
    const BLOCK_LEN: u32; // Length of block

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
        sentinel_chr_index: u64, // Sentinel character index in bwt_text - save for locate
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
            sentinel_chr_index,
            chr_with_sentinel_count,
            _padding: 0,
            rank_checkpoints_len,
            blocks_len,
        }
    }
    pub fn write_to_blob<P: Position, B: Block>(
        &self,
        bwt_text: Vec<u8>, // burrow-wheeler transformed text
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

        let rank_checkpoints_size = self.rank_checkpoints_len as usize * std::mem::size_of::<P>();
        let blocks_size = self.blocks_len as usize * std::mem::size_of::<B>();

        let (rank_checkpoints_blob, blocks_blob) = {
            let (left, right) = blob.split_at_mut(rank_checkpoints_size);
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
