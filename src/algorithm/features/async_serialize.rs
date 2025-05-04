use std::pin::Pin;

use crate::core::{Position, AsyncSerialize};
use super::{LtFmIndex, ChrIdxTable, SuffixArray, CountArray, Bwm, Block};
use capwriter::{AsyncSave, AsyncLoad};

impl<P: Position, B: Block<P>> LtFmIndex<P, B> {
    pub fn async_save_to<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), std::io::Error>> + Send where
        W: tokio::io::AsyncWrite + Send,
    {
        async move {
            // text_len
            self.text_len.as_u64().save_as_ne(writer.as_mut()).await?;
            // chr_idx_table
            self.chr_idx_table.async_save_to(writer.as_mut()).await?;
            // suffix_array
            self.suffix_array.async_save_to(writer.as_mut()).await?;
            // count_array
            self.count_array.async_save_to(writer.as_mut()).await?;
            // bwm
            self.bwm.async_save_to(writer.as_mut()).await?;
            Ok(())
        }
    }
    pub fn async_load_from<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, std::io::Error>> + Send where
        R: tokio::io::AsyncRead + Send,
        Self: Sized
    {
        async move {
            let text_len = P::from_u64(u64::load_as_ne(reader.as_mut()).await?);
            let chr_idx_table = ChrIdxTable::async_load_from(reader.as_mut()).await?;
            let suffix_array = SuffixArray::async_load_from(reader.as_mut()).await?;
            let count_array = CountArray::async_load_from(reader.as_mut()).await?;
            let bwm = Bwm::async_load_from(reader.as_mut()).await?;
            Ok(Self {
                text_len,
                chr_idx_table,
                suffix_array,
                count_array,
                bwm,
            })
        }
    }
}
