use crate::core::{Position, Serialize};
use super::{Bwm, Block};
use capwriter::{Save, Load};

impl<T, B> Serialize for Bwm<T, B> where
    T: Position,
    B: Block<T>,
{
    #[allow(unused_must_use)]
    fn save_to<W>(&self, writer: &mut W) -> Result<(), std::io::Error> where
        W: std::io::Write,
    {
        // primary_index
        self.primary_index.as_u64().save_as_ne(writer)?;
        // chr_count
        self.chr_count.save_as_ne(writer)?;
        // rank_checkpoints
        self.rank_checkpoints.save_as_ne(writer)?;
        // blocks
        let blocks_len = self.blocks.len() as u64;
        blocks_len.save_as_ne(writer)?;
        let casted_blocks = bytemuck::cast_slice(&self.blocks);
        writer.write_all(casted_blocks)?;

        Ok(())
    }
    fn load_from<R>(reader: &mut R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized,
    {
        // primary_index
        let primary_index = u64::load_as_ne(reader)?;
        // chr_count
        let chr_count = u32::load_as_ne(reader)?;
        // rank_checkpoints
        let rank_checkpoints = Vec::<T>::load_as_ne(reader)?;
        // blocks length
        let blocks_len = u64::load_as_ne(reader)? as usize;
        let mut blocks = vec![B::zeroed(); blocks_len];
        // Read from reader
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut blocks);
        reader.read_exact(casted_buffer)?;
        
        Ok(Self {
            primary_index: T::from_u64(primary_index),
            chr_count,
            rank_checkpoints,
            blocks,
        })
    }
    fn encoded_len(&self) -> usize {
        let block_bytes: &[u8] = bytemuck::cast_slice(&self.blocks);
        12 // primary_index(8) + chr_count(4)
        + self.rank_checkpoints.encoded_len() // rank_checkpoints
        + 8 // blocks_len
        + block_bytes.len() // blocks
    }
}

