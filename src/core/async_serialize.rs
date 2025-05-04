use std::pin::Pin;
use std::future::Future;

pub trait AsyncSerialize {
    fn async_save_to<W>(&self, writer: Pin<&mut W>) -> impl Future<Output = Result<(), std::io::Error>> + Send where
        W: tokio::io::AsyncWrite + Send;
    fn async_load_from<R>(reader: Pin<&mut R>) -> impl Future<Output = Result<Self, std::io::Error>> + Send where
        R: tokio::io::AsyncRead + Send,
        Self: Sized;
}