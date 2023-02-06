use thiserror::Error;

/// Errors thats can occur in this crate
#[derive(Debug, Error)]
pub enum LtFmIndexError {
    #[error(transparent)]
    IoError(#[from] IoError),
    #[error(transparent)]
    BuildError(#[from] BuildError),
}

pub use crate::structures_dep::IoError;
pub use crate::builder::BuildError;
