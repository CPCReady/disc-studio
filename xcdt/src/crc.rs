#![allow(dead_code)]
/// CRC-16 polynomial: X^16 + X^12 + X^5 + 1 (as used by Amstrad CPC firmware)
const CRC_POLY: u16 = 4129;

/// Update a running CRC with one byte. Initial CRC must be 0xFFFF.
pub fn update(crc: u16, byte: u8) -> u16 {
    let mut aux = crc ^ ((byte as u16) << 8);
    for _ in 0..8 {
        if aux & 0x8000 != 0 {
            aux = aux.wrapping_shl(1) ^ CRC_POLY;
        } else {
            aux = aux.wrapping_shl(1);
        }
    }
    aux
}

/// Compute the CRC-16 of a byte slice and return the 2-byte result (high, low)
/// XOR'd with 0xFF each (inverted), as the CPC tape format requires.
pub fn compute_inverted(data: &[u8]) -> [u8; 2] {
    let mut crc: u16 = 0xFFFF;
    for &b in data {
        crc = update(crc, b);
    }
    [(crc >> 8) as u8 ^ 0xFF, crc as u8 ^ 0xFF]
}

/// Compute the CRC-16 of a byte slice and return the 2-byte result XOR'd with
/// 0xFFFF (pure inversion as bitstream format uses). Returns (high, low).
pub fn compute_xor_ffff(data: &[u8]) -> [u8; 2] {
    let mut crc: u16 = 0xFFFF;
    for &b in data {
        crc = update(crc, b);
    }
    crc ^= 0xFFFF;
    [(crc >> 8) as u8, crc as u8]
}

/// Verify that the CRC of a 256-byte chunk (followed by 2 inverted CRC bytes) is valid.
pub fn verify_chunk(chunk_and_crc: &[u8]) -> bool {
    if chunk_and_crc.len() < 258 {
        return false;
    }
    let data = &chunk_and_crc[..256];
    let stored = &chunk_and_crc[256..258];
    let expected = compute_inverted(data);
    stored == expected
}
