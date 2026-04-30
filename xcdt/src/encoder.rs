#![allow(dead_code)]
/// MSB-first bitstream writer, used for Pure Data blocks (TZX ID 0x14).
pub struct BitStream {
    data: Vec<u8>,
    bit_pos: u8, // 0..7, next bit to write (7=MSB of current byte)
}

impl BitStream {
    pub fn new() -> Self {
        BitStream {
            data: vec![0u8],
            bit_pos: 7,
        }
    }

    /// Write a single bit (0 or 1), MSB first.
    pub fn write_bit(&mut self, bit: u8) {
        let last = self.data.last_mut().unwrap();
        if bit != 0 {
            *last |= 1 << self.bit_pos;
        } else {
            *last &= !(1 << self.bit_pos);
        }
        if self.bit_pos == 0 {
            self.data.push(0u8);
            self.bit_pos = 7;
        } else {
            self.bit_pos -= 1;
        }
    }

    /// Write one byte (8 bits), MSB first.
    pub fn write_byte(&mut self, byte: u8) {
        for shift in (0..8).rev() {
            self.write_bit((byte >> shift) & 1);
        }
    }

    /// Consume the bitstream and return the packed bytes.
    /// The last byte may be partially filled (remaining bits are 0).
    pub fn into_bytes(mut self) -> Vec<u8> {
        // Remove trailing empty byte if we're exactly on a byte boundary
        if self.bit_pos == 7 && self.data.last() == Some(&0) {
            // Only pop if it was added as padding (not if it's meaningful data)
            // Since we always push a new 0 after completing a byte,
            // the trailing 0 is padding if bit_pos == 7.
            self.data.pop();
        }
        self.data
    }

    /// Return the number of bytes used so far (including partial final byte).
    pub fn byte_count(&self) -> usize {
        self.data.len()
    }
}

impl Default for BitStream {
    fn default() -> Self {
        Self::new()
    }
}

use crate::crc;
use crate::timing::CPC_DATA_CHUNK_SIZE;

/// Encode raw data into a Pure Data bitstream payload.
///
/// Format:
///   [2048 × bit 1 (pilot)]
///   [1 × bit 0 (sync)]
///   [sync_byte (8 bits)]
///   for each 256-byte chunk:
///     [256 bytes of data, zero-padded if last chunk is short]
///     [2-byte CRC, inverted (XOR 0xFFFF)]
///   [32 × bit 1 (trailer)]
pub fn encode_pure_data(sync_byte: u8, data: &[u8]) -> Vec<u8> {
    use crate::timing::CPC_PILOT_TONE_NUM_WAVES;

    let mut bs = BitStream::new();

    // Pilot tone: 2048 '1' bits
    for _ in 0..CPC_PILOT_TONE_NUM_WAVES {
        bs.write_bit(1);
    }
    // Sync bit
    bs.write_bit(0);
    // Sync byte
    bs.write_byte(sync_byte);

    // Chunked data
    let num_chunks = (data.len() + CPC_DATA_CHUNK_SIZE - 1) / CPC_DATA_CHUNK_SIZE;
    for i in 0..num_chunks {
        let start = i * CPC_DATA_CHUNK_SIZE;
        let end = (start + CPC_DATA_CHUNK_SIZE).min(data.len());
        let slice = &data[start..end];

        // Pad to 256 bytes
        let mut chunk = [0u8; 256];
        chunk[..slice.len()].copy_from_slice(slice);

        // Write bytes and accumulate CRC
        let mut crc_val: u16 = 0xFFFF;
        for &b in &chunk {
            crc_val = crc::update(crc_val, b);
            bs.write_byte(b);
        }

        // Write CRC inverted (XOR 0xFFFF) as (high, low)
        crc_val ^= 0xFFFF;
        bs.write_byte((crc_val >> 8) as u8);
        bs.write_byte(crc_val as u8);
    }

    // Trailer: 32 '1' bits
    for _ in 0..32 {
        bs.write_bit(1);
    }

    bs.into_bytes()
}
