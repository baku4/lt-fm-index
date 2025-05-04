use std::pin::Pin;

use tokio::io::{AsyncReadExt, AsyncWriteExt};

use crate::core::AsyncSerialize;
use super::ChrIdxTable;

impl AsyncSerialize for ChrIdxTable {
    fn async_save_to<W>(&self, mut writer: Pin<&mut W>) -> impl Future<Output = Result<(), std::io::Error>> + Send where
        W: tokio::io::AsyncWrite + Send,
    {
        async move {
            writer.as_mut().write_all(&self.0).await?;
            Ok(())
        }
    }
    fn async_load_from<R>(mut reader: Pin<&mut R>) -> impl Future<Output = Result<Self, std::io::Error>> + Send where
        R: tokio::io::AsyncRead + Send,
        Self: Sized 
    {
        async move {
            let mut buf = [0; 256];
            reader.as_mut().read_exact(&mut buf).await?;
            Ok(Self(buf))
        }
    }
}
