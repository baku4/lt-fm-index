use std::pin::Pin;

use crate::core::{Position, AsyncSerialize};
use super::CountArray;
use capwriter::{AsyncSave, AsyncLoad};

impl<P: Position> AsyncSerialize for CountArray<P> {
    fn async_save_to<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), std::io::Error>> + Send where
        W: tokio::io::AsyncWrite + Send,
    {
        async move {
            // kmer_size
            self.kmer_size.save_as_ne(writer.as_mut()).await?;

            // count_table
            self.count_table.save_as_ne(writer.as_mut()).await?;

            // kmer_count_table
            self.kmer_count_table.save_as_ne(writer.as_mut()).await?;

            // multiplier
            self.multiplier.save_as_ne(writer.as_mut()).await?;

            Ok(())
        }
    }
    fn async_load_from<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, std::io::Error>> + Send where
        R: tokio::io::AsyncRead + Send,
        Self: Sized,
    {
        async move {
            // kmer_size
            let kmer_size = u32::load_as_ne(reader.as_mut()).await?;

            // count_table
            let count_table = Vec::<P>::load_as_ne(reader.as_mut()).await?;

            // kmer_count_table
            let kmer_count_table = Vec::<P>::load_as_ne(reader.as_mut()).await?;

            // multiplier
            let multiplier = Vec::<usize>::load_as_ne(reader.as_mut()).await?;

            Ok(Self {
                kmer_size,
                count_table,
                kmer_count_table,
                multiplier,
            })
        }
    }
}
