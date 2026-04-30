#![allow(dead_code)]
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CDT/TZX error: {0}")]
    Tzx(String),

    #[error("AMSDOS header error: {0}")]
    Amsdos(String),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
