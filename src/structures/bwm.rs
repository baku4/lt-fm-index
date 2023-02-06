use std::marker::PhantomData;

use crate::core::{
    Text,
    EndianType, ReadBytesExt, WriteBytesExt, Serializable,
};

use super::TextEncoder;

// Burrows-Wheeler Matrix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bwm<T: TextEncoder, B: BlockVector> {
    primary_index: u64,
    blocks: Vec<B>,
    phantom_data: PhantomData<T>,
}
pub struct BwtBlock<T: TextEncoder> {
    rank_check_point: [u64; T::CHR_COUNT],
    bwt_vector: [u64; 4],
}

pub trait BwtBlock {

}

// Bwt Implementations
impl<T: TextEncoder, B: BwtBlock> Bwm<T, B> {
    // Build
    fn new(bwt_text: Text, pidx: u64) -> Self {
        let blocks: Vec<V> = Self::new_with_bwt_text(bwt_text);

        Self {
            primary_index: pidx,
            blocks: blocks,
        }
    }
    fn new_with_bwt_text(bwt_text: Text) -> Vec<V> {
        let mut chunk_count = bwt_text.len() / V::BLOCK_SEG_LEN as usize;
        let rem = bwt_text.len() % V::BLOCK_SEG_LEN as usize;
        
        let last_offset = if rem == 0 {
            chunk_count += 1;
            rem
        } else {
            V::BLOCK_SEG_LEN as usize - rem
        };

        let mut rank_checkpoint = W::empty_rank_check_point();
        let mut blocks: Vec<W> = Vec::with_capacity(chunk_count);

        bwt_text.chunks(W::BLOCK_SEG_LEN as usize).for_each(|text_chunk| {
            let block_rank_checkpoint = rank_checkpoint.clone();
            
            let bwt_vector = W::encoding_text_chunk(text_chunk, &mut rank_checkpoint);

            let block = W::new(block_rank_checkpoint, bwt_vector);
            
            blocks.push(block);
        });

        if last_offset == 0 {
            let last_block = W::new_last(rank_checkpoint);
            blocks.push(last_block);
        } else {
            let last_block = blocks.last_mut().unwrap();
            last_block.add_offset(last_offset);
        }

        blocks
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


// impl<B> Serializable for Bwm<B> where
//     B: BwtBlockInterface + bytemuck::Pod,
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
//     fn size_of(&self) -> usize {
//         let casted_blocks: &[u8] = bytemuck::cast_slice(&self.blocks);
//         16 // primary_index(8) + blocks_len(8)
//         + casted_blocks.len() // casted blocks
//     }
// }


// BwtBlock Requirements
pub trait BwtBlockInterface {
    const BLOCK_SEG_LEN: u64;
    
    type RankCheckPoint;
    type BwtVector;

    fn empty_rank_check_point() -> Self::RankCheckPoint;
    fn encoding_text_chunk(text_chunk: &[u8], rank_check_point: &mut Self::RankCheckPoint) -> Self::BwtVector;
    fn new(block_rank_check_point: Self::RankCheckPoint, bwt_vectors: Self::BwtVector) -> Self;
    fn new_last(rank_check_point: Self::RankCheckPoint) -> Self;
    fn add_offset(&mut self, last_offset: usize);

    fn get_chridx_and_rank_of_rem(&self, rem: u64) -> (usize, u64);
    fn get_rank_of_chridx_and_rem(&self, chridx: usize, rem: u64) -> u64;
}
