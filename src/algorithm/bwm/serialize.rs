use crate::core::{Position, Serialize, EndianType, WriteBytesExt, ReadBytesExt};
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
        writer.write_u64::<EndianType>(self.primary_index.as_u64())?;
        // chr_count
        writer.write_u32::<EndianType>(self.chr_count)?;
        // rank_checkpoints
        self.rank_checkpoints.save_to(writer)?;
        // blocks
        let blocks_len = self.blocks.len() as u64;
        writer.write_u64::<EndianType>(blocks_len)?;
        let casted_blocks = bytemuck::cast_slice(&self.blocks);
        writer.write_all(casted_blocks)?;

        Ok(())
    }
    fn load_from<R>(reader: &mut R) -> Result<Self, std::io::Error> where
        R: std::io::Read,
        Self: Sized,
    {
        // primary_index
        let primary_index = T::from_u64(reader.read_u64::<EndianType>()?);
        // chr_count
        let chr_count = reader.read_u32::<EndianType>()?;
        // rank_checkpoints
        let rank_checkpoints = Vec::<T>::load_from(reader)?;
        // blocks length
        let blocks_len = reader.read_u64::<EndianType>()? as usize;
        let mut blocks = vec![B::zeroed(); blocks_len];
        // Read from reader
        let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut blocks);
        reader.read_exact(casted_buffer)?;
        
        Ok(Self {
            primary_index,
            chr_count,
            rank_checkpoints,
            blocks,
        })
    }
    fn to_be_saved_size(&self) -> usize {
        12 // primary_index(8) + chr_count(4)
        + self.rank_checkpoints.to_be_saved_size() // rank_checkpoints
        + self.blocks.to_be_saved_size() // blocks
    }
}
