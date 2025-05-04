use std::pin::Pin;

use crate::core::{Position, AsyncSerialize};
use super::SuffixArray;
use capwriter::{AsyncSave, AsyncLoad};

impl<P: Position> AsyncSerialize for SuffixArray<P> {
    fn async_save_to<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), std::io::Error>> + Send where
        W: tokio::io::AsyncWrite + Send,
    {
        async move {
            self.sampling_ratio.as_u64().save_as_ne(writer.as_mut()).await?;

            self.array.save_as_ne(writer.as_mut()).await?;

            Ok(())
        }
    }
    fn async_load_from<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, std::io::Error>> + Send where
        R: tokio::io::AsyncRead + Send,
        Self: Sized,
    {
        async move {
            let sampling_ratio = P::from_u64(u64::load_as_ne(reader.as_mut()).await?);

            let array = Vec::<P>::load_as_ne(reader.as_mut()).await?;

            Ok(Self{
                sampling_ratio,
                array,
            })
        }
    }
}
