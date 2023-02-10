use crate::core::{
    Text,
    Serialize, EndianType, WriteBytesExt, ReadBytesExt,
};

// Burrows-Wheeler Matrix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bwm<B: BwtBlock> {
    primary_index: u64,
    chr_count: usize,
    rank_checkpoints: Vec<u64>,
    blocks: Vec<B>,
}
pub trait BwtBlock: Sized + std::fmt::Debug {
    const BLOCK_LEN: u64;

    fn vectorize(bwt_text: &[u8], rank_pre_counts: &mut Vec<u64>) -> Self;
    fn get_remain_count_of(&self, rem: u64, chridx: u8) -> u64;
    fn get_chridx_of(&self, rem: u64) -> u8;    
}

// Bwm Implementations
impl<B: BwtBlock> Bwm<B> {
    // Build
    #[inline]
    pub fn new(bwt_text: Text, pidx: u64, chr_count: usize) -> Self {
        let block_len = B::BLOCK_LEN;
        let mut chunk_count = bwt_text.len() / block_len as usize;
        let rem = bwt_text.len() % block_len as usize;
        
        let last_offset = if rem == 0 {
            chunk_count += 1;
            rem
        } else {
            block_len as usize - rem
        };

        let mut rank_checkpoints = Vec::with_capacity(chunk_count * chr_count);
        let mut rank_pre_counts = vec![0; chr_count];
        let mut blocks: Vec<B> = Vec::with_capacity(chunk_count);

        bwt_text.chunks(block_len as usize).for_each(|text_chunk| {
            rank_checkpoints.extend_from_slice(&rank_pre_counts);
            let block = B::vectorize(text_chunk, &mut rank_pre_counts);
            blocks.push(block);
        });

        Self {
            primary_index: pidx,
            chr_count,
            rank_checkpoints,
            blocks: blocks,
        }
    }
    // Locate
    #[inline]
    pub fn get_next_rank(&self, mut pos: u64, chridx: u8) -> u64 {
        if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / B::BLOCK_LEN;
        let rem = pos % B::BLOCK_LEN;

        let rank_idx = quot as usize * self.chr_count + chridx as usize;
        let rank_precount = self.rank_checkpoints[rank_idx];
        if rem == 0 {
            rank_precount
        } else {
            let rem_count = self.blocks[quot as usize].get_remain_count_of(rem, chridx);
            rank_precount + rem_count
        }
    }
    #[inline]
    pub fn get_pre_rank_and_chridx(&self, mut pos: u64) -> Option<(u64, u8)> {
        if pos == self.primary_index - 1 {
            return None;
        } else if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / B::BLOCK_LEN;
        let rem = pos % B::BLOCK_LEN;
        
        let block = &self.blocks[quot as usize];
        let chridx = block.get_chridx_of(rem);

        let rank_idx = quot as usize * self.chr_count + chridx as usize;
        let rank_precount = self.rank_checkpoints[rank_idx];
        if rem == 0 {
            Some((rank_precount, chridx))
        } else {
            let rem_count = block.get_remain_count_of(rem, chridx);
            Some((rank_precount + rem_count, chridx))
        }
    }
}

// impl<B> Serialize for Bwm<B> where
//     B: BwtBlock + bytemuck::Pod,
// {
//     #[allow(unused_must_use)]
//     fn save_to<W>(&self, mut writer: W) -> Result<(), std::io::Error> where
//         W: std::io::Write,
//     {
//         // primary_index
//         writer.write_u64::<EndianType>(self.primary_index)?;
//         // blocks length
//         let blocks_len = self.blocks.len() as u64;
//         writer.write_u64::<EndianType>(blocks_len)?;
//         // casted_blocks
//         let casted_blocks = bytemuck::cast_slice(&self.blocks);
//         writer.write_all(casted_blocks)?;

//         Ok(())
//     }
//     fn load_from<R>(mut reader: R) -> Result<Self, std::io::Error> where
//         R: std::io::Read,
//         Self: Sized,
//     {
//         // primary_index
//         let primary_index = reader.read_u64::<EndianType>()?;
//         // blocks length
//         let blocks_len = reader.read_u64::<EndianType>()? as usize;
//         let mut blocks = vec![B::zeroed(); blocks_len];
//         // Read from reader
//         let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut blocks);
//         reader.read_exact(casted_buffer)?;
        
//         Ok(Self {
//             primary_index,
//             blocks,
//         })
//     }
//     fn estimate_size(&self) -> usize {
//         let casted_blocks: &[u8] = bytemuck::cast_slice(&self.blocks);
//         16 // primary_index(8) + blocks_len(8)
//         + casted_blocks.len() // casted blocks
//     }
// }