mod header;
mod types;

pub use header::AmsdosHeader;
pub use types::FileType;

pub fn has_header(data: &[u8]) -> bool {
    if data.len() < AmsdosHeader::SIZE {
        return false;
    }
    AmsdosHeader::from_bytes(data).is_ok()
}

pub fn calculate_checksum(data: &[u8]) -> u16 {
    data.iter().take(67).map(|&b| b as u16).sum()
}
