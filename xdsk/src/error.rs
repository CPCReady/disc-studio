use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum DskError {
    #[error("Invalid DSK format: {0}")]
    InvalidFormat(String),

    #[error("File not found in DSK: {0}")]
    FileNotFound(String),

    #[error("File already exists: {0}")]
    FileExists(String),

    #[error("No free directory entries")]
    NoFreeDirEntry,

    #[error("No free blocks available")]
    NoFreeBlocks,

    #[error("Invalid AMSDOS header")]
    InvalidAmsdosHeader,

    #[error("Invalid track number: {0}")]
    InvalidTrack(u8),

    #[error("Invalid sector number: {0}")]
    InvalidSector(u8),

    #[error("Disk is full")]
    DiskFull,

    #[error("File too large: {0} bytes")]
    FileTooLarge(usize),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Unsupported DSK format: {0}")]
    UnsupportedFormat(String),
}

pub type Result<T> = std::result::Result<T, DskError>;
