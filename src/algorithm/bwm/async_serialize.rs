use crate::core::{Position, AsyncSerialize};
use super::{Bwm, Block};
use std::pin::Pin;
use std::future::Future;
use capwriter::{AsyncSave, AsyncLoad};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl<T, B> AsyncSerialize for Bwm<T, B> where
    T: Position,
    B: Block<T>,
{
    fn async_save_to<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), std::io::Error>> + Send where
        W: tokio::io::AsyncWrite + Send,
    {
        async move {
            // primary_index
            self.primary_index.as_u64().save_as_ne(writer.as_mut()).await?;
            // chr_count
            self.chr_count.save_as_ne(writer.as_mut()).await?;
            // rank_checkpoints
            self.rank_checkpoints.save_as_ne(writer.as_mut()).await?;
            // blocks
            let blocks_len = self.blocks.len() as u64;
            blocks_len.save_as_ne(writer.as_mut()).await?;
            let casted_blocks = bytemuck::cast_slice(&self.blocks);
            writer.write_all(casted_blocks).await?;

            Ok(())
        }
    }
    fn async_load_from<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, std::io::Error>> + Send where
        R: tokio::io::AsyncRead + Send,
    {
        async move {
            // primary_index
            let primary_index = u64::load_as_ne(reader.as_mut()).await?;
            // chr_count
            let chr_count = u32::load_as_ne(reader.as_mut()).await?;
            // rank_checkpoints
            let rank_checkpoints = Vec::<T>::load_as_ne(reader.as_mut()).await?;
            // blocks length
            let blocks_len = u64::load_as_ne(reader.as_mut()).await? as usize;
            let mut blocks = vec![B::zeroed(); blocks_len];
            // Read from reader
            let casted_buffer: &mut [u8] = bytemuck::cast_slice_mut(&mut blocks);
            reader.as_mut().read_exact(casted_buffer).await?;
            
            Ok(Self {
                primary_index: T::from_u64(primary_index),
                chr_count,
                rank_checkpoints,
                blocks,
            })
        }
    }
}