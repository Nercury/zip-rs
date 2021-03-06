//! Error types that can be emitted from this library

use std::io;

use thiserror::Error;

/// Generic result type with ZipError as its error variant
pub type ZipResult<T> = Result<T, ZipError>;

/// Error type for Zip
#[derive(Debug, Error)]
pub enum ZipError {
    /// An Error caused by I/O
    #[error(transparent)]
    Io(#[from] io::Error),

    /// This file is probably not a zip archive
    #[error("invalid Zip archive")]
    InvalidArchive(&'static str),

    /// This archive is not supported
    #[error("unsupported Zip archive")]
    UnsupportedArchive(&'static str),

    /// The requested file could not be found in the archive
    #[error("specified file not found in archive")]
    FileNotFound,
}
