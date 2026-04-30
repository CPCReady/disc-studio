#![allow(dead_code)]
/// Size of the AMSDOS file header as stored in a tape header block.
pub const TAPE_HEADER_SIZE: usize = 64;

/// Size of the full AMSDOS disk header (128 bytes). The tape uses only the
/// first 64 bytes, but the checksum covers bytes 0-66 and is stored at 67-68.
pub const AMSDOS_DISK_HEADER_SIZE: usize = 128;

/// File type constants.
pub mod file_type {
    pub const BASIC: u8 = 0;
    pub const BINARY: u8 = 2;
}

/// Decoded AMSDOS tape header (64 bytes).
#[derive(Debug, Clone)]
pub struct AmsdosHeader {
    /// File name, padded with spaces/zeros (16 bytes).
    pub name: [u8; 16],
    /// Block number (1-based).
    pub block_number: u8,
    /// 0xFF if this is the last block, 0x00 otherwise.
    pub last_block: u8,
    /// File type: 0=BASIC, 2=Binary.
    pub file_type: u8,
    /// Size of data in this block.
    pub data_length: u16,
    /// Load address in memory.
    pub load_address: u16,
    /// 0xFF if this is the first block, 0x00 otherwise.
    pub first_block: u8,
    /// Total logical length of the file.
    pub logical_length: u16,
    /// Execution address.
    pub exec_address: u16,
}

impl AmsdosHeader {
    /// Parse a 64-byte tape header block.
    pub fn from_bytes(b: &[u8]) -> Option<Self> {
        if b.len() < TAPE_HEADER_SIZE {
            return None;
        }
        let mut name = [0u8; 16];
        name.copy_from_slice(&b[0..16]);

        Some(AmsdosHeader {
            name,
            block_number:  b[16],
            last_block:    b[17],
            file_type:     b[18],
            data_length:   u16::from_le_bytes([b[19], b[20]]),
            load_address:  u16::from_le_bytes([b[21], b[22]]),
            first_block:   b[23],
            logical_length: u16::from_le_bytes([b[24], b[25]]),
            exec_address:  u16::from_le_bytes([b[26], b[27]]),
        })
    }

    /// Serialize back to 64 bytes (zeros in reserved area).
    pub fn to_bytes(&self) -> [u8; TAPE_HEADER_SIZE] {
        let mut b = [0u8; TAPE_HEADER_SIZE];
        b[0..16].copy_from_slice(&self.name);
        b[16] = self.block_number;
        b[17] = self.last_block;
        b[18] = self.file_type;
        b[19] = self.data_length as u8;
        b[20] = (self.data_length >> 8) as u8;
        b[21] = self.load_address as u8;
        b[22] = (self.load_address >> 8) as u8;
        b[23] = self.first_block;
        b[24] = self.logical_length as u8;
        b[25] = (self.logical_length >> 8) as u8;
        b[26] = self.exec_address as u8;
        b[27] = (self.exec_address >> 8) as u8;
        b
    }

    /// Returns the file name as a trimmed ASCII string.
    pub fn name_str(&self) -> String {
        let end = self.name.iter().position(|&b| b == 0).unwrap_or(16);
        let s = &self.name[..end];
        String::from_utf8_lossy(s).trim().to_string()
    }

    /// Returns the file type as a human-readable string.
    pub fn type_str(&self) -> &'static str {
        match self.file_type {
            0 => "BASIC",
            1 => "PROTECTED",
            2 => "BINARY",
            _ => "UNKNOWN",
        }
    }
}

/// Compute AMSDOS checksum (simple sum of bytes 0..67) as stored in a
/// full 128-byte disk header.
pub fn amsdos_checksum(data: &[u8]) -> u16 {
    let limit = data.len().min(67);
    data[..limit].iter().map(|&b| b as u16).sum()
}

/// Returns true if the 128-byte buffer starts with a valid AMSDOS header
/// (checksum at bytes 67-68 matches sum of bytes 0-66).
pub fn has_amsdos_header(data: &[u8]) -> bool {
    if data.len() < 69 {
        return false;
    }
    let calculated = amsdos_checksum(data);
    let stored = u16::from_le_bytes([data[67], data[68]]);
    calculated == stored
}
