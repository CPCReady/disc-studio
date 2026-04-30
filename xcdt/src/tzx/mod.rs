pub mod blocks;
mod reader;
mod writer;

pub use blocks::*;
pub use reader::read_file;
pub use writer::{append_file, write_file};

/// TZX/CDT version used when creating new files.
pub const TZX_VERSION_MAJOR: u8 = 1;
pub const TZX_VERSION_MINOR: u8 = 10;

/// An in-memory TZX/CDT file.
#[derive(Debug, Clone)]
pub struct TzxFile {
    pub version_major: u8,
    pub version_minor: u8,
    pub blocks: Vec<TzxBlock>,
}

impl TzxFile {
    pub fn new() -> Self {
        TzxFile {
            version_major: TZX_VERSION_MAJOR,
            version_minor: TZX_VERSION_MINOR,
            blocks: Vec::new(),
        }
    }

    pub fn push(&mut self, block: TzxBlock) {
        self.blocks.push(block);
    }
}

impl Default for TzxFile {
    fn default() -> Self {
        Self::new()
    }
}
