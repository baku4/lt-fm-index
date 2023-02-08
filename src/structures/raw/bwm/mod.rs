use crate::core::{
    Text,
    EndianType, ReadBytesExt, WriteBytesExt, Serializable,
};
use super::ChrIdxTable;

// Burrows-Wheeler Matrix
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bwm<B: BwtBlock> {
    primary_index: u64,
    blocks: Vec<B>,
}

pub trait BwtBlock: Sized + std::fmt::Debug {
    const BIT_LEN: u64;

    fn new_with_bwt_text(bwt_text: Text) -> Vec<Self>;
    fn get_rank(&self, rem: u64, chridx: u8) -> u64;
    fn get_rank_and_chridx_of_rem(&self, rem: u64) -> (u64, u8);
}

// Bwt Implementations
impl<B: BwtBlock> Bwm<B> {
    // Build
    pub fn new(bwt_text: Text, pidx: u64) -> Self {
        let blocks: Vec<B> = B::new_with_bwt_text(bwt_text);

        Self {
            primary_index: pidx,
            blocks: blocks,
        }
    }
    pub fn get_next_rank_of_pos_and_chridx(&self, mut pos: u64, chridx: u8) -> u64 {
        if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / B::BIT_LEN;
        let rem = pos % B::BIT_LEN;

        self.blocks[quot as usize].get_rank(rem, chridx)
    }

    pub fn get_pre_rank_and_chridx_of_pos(&self, mut pos: u64) -> Option<(u64, u8)> {
        if pos == self.primary_index - 1 {
            return None;
        } else if pos < self.primary_index {
            pos += 1;
        }
        let quot = pos / B::BIT_LEN;
        let rem = pos % B::BIT_LEN;

        let (rank, chridx) = self.blocks[quot as usize].get_rank_and_chridx_of_rem(rem);
        Some((rank, chridx))
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
