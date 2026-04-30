// MIT License — Copyright (c) Destroyer 2026.
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("DSK error: {0}")]
    Dsk(String),

    #[error("CPR error: {0}")]
    Cpr(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;
