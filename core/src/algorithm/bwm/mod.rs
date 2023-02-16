use crate::core::{
    TextLength,
    Serialize, EndianType, WriteBytesExt, ReadBytesExt,
};
use capwriter::{Saveable, Loadable};

mod blocks;

// Burrows-Wheeler Matrix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bwm<T: TextLength, B: Block<T>> {
    primary_index: T,
    chr_count: u32,
    rank_checkpoints: Vec<T>,
    blocks: Vec<B>,
}
pub trait Block<T: TextLength>: Sized {
    const BLOCK_LEN: u32;
    // Build
    fn vectorize(bwt_text: &[u8], rank_pre_counts: &mut Vec<T>) -> Self;
    fn empty() -> Self;
    fn shift_last_offset(&mut self, offset: u32);
    // Locate
    fn get_remain_count_of(&self, rem: u32, chridx: u8) -> u32;
    fn get_chridx_of(&self, rem: u32) -> u8;
}

// Bwm Implementations
impl<T: TextLength, B: Block<T>> Bwm<T, B> {
    // Build
    #[inline]
    pub fn new(bwt_text: Vec<u8>, pidx: T, chr_count: u32) -> Self {
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
        let mut rank_pre_counts = vec![T::ZERO; chr_count as usize];
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
    pub fn get_next_rank(&self, mut pos: T, chridx: u8) -> T {
        if pos < self.primary_index {
            pos += T::ONE;
        }
        let quot = pos.as_u32() / B::BLOCK_LEN;
        let rem = pos.as_u32() % B::BLOCK_LEN;

        let rank_idx = quot.as_u32() * self.chr_count + chridx as u32;
        let rank_precount = self.rank_checkpoints[rank_idx as usize];
        if rem == 0 {
            rank_precount
        } else {
            let rem_count = self.blocks[quot.as_usize()].get_remain_count_of(rem, chridx);
            rank_precount + T::from_u32(rem_count)
        }
    }
    #[inline]
    pub fn get_pre_rank_and_chridx(&self, mut pos: T) -> Option<(T, u8)> {
        if pos == self.primary_index - T::ONE {
            return None;
        } else if pos < self.primary_index {
            pos += T::ONE;
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
            Some((rank_precount + T::from_u32(rem_count), chridx))
        }
    }

    pub fn chr_count(&self) -> u32 {
        self.chr_count
    }
}

// impl<B> Serialize for Bwm<B> where
//     B: Block + bytemuck::Pod,
// {
//     #[allow(unused_must_use)]
//     fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
//         W: std::io::Write,
//     {
//         // primary_index
//         writer.write_u64::<EndianType>(self.primary_index as u64)?;
//         // chr_count
//         writer.write_u64::<EndianType>(self.chr_count as u64)?;
//         // rank_checkpoints
//         self.rank_checkpoints.save_to(&mut writer)?;
//         // blocks
//         let blocks_len = self.blocks.len() as u64;
//         writer.write_u64::<EndianType>(blocks_len)?;
//         let casted_blocks = bytemuck::cast_slice(&self.blocks);
//         writer.write_all(casted_blocks)?;

//         Ok(())
//     }
//     fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
//         R: std::io::Read,
//         Self: Sized,
//     {
//         // primary_index
//         let primary_index = reader.read_u64::<EndianType>()? as TextLen;
//         // chr_count
//         let chr_count = reader.read_u64::<EndianType>()? as usize;
//         // rank_checkpoints
//         let rank_checkpoints = Vec::<TextLength>::load_from(&mut reader)?;
//         // blocks length
//         let blocks_len = reader.read_u64::<EndianType>()? as usize;
//         let mut blocks = vec![B::zeroed(); blocks_len];
//         // Read from reader
//         let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut blocks);
//         reader.read_exact(casted_buffer)?;
        
//         Ok(Self {
//             primary_index,
//             chr_count,
//             rank_checkpoints,
//             blocks,
//         })
//     }
//     fn estimate_size(&self) -> usize {
//         let casted_blocks: &[u8] = bytemuck::cast_slice(&self.blocks);
//         24 // primary_index(8) + chr_count(8) + blocks_len(8)
//         + self.rank_checkpoints.size_of() // rank_checkpoints
//         + casted_blocks.len() // casted blocks
//     }
// }
