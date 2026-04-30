use std::path::Path;

use crate::error::{Error, Result};
use super::blocks::*;
use super::TzxFile;

/// Magic signature for TZX/CDT files.
pub const TZX_MAGIC: &[u8; 8] = b"ZXTape!\x1a";

pub fn read_file(path: &Path) -> Result<TzxFile> {
    let data = std::fs::read(path)?;
    parse(&data).map_err(|e| Error::Tzx(format!("{}: {}", path.display(), e)))
}

/// Parse a TZX/CDT byte slice into a `TzxFile`.
pub fn parse(data: &[u8]) -> Result<TzxFile> {
    if data.len() < 10 {
        return Err(Error::Tzx("File too short to be a valid TZX/CDT".into()));
    }
    if &data[..8] != TZX_MAGIC {
        return Err(Error::Tzx(
            "Invalid magic bytes — not a TZX/CDT file".into(),
        ));
    }
    let version_major = data[8];
    let version_minor = data[9];

    let mut blocks = Vec::new();
    let mut pos = 10usize;

    while pos < data.len() {
        let block_id = data[pos];
        pos += 1;

        let block = match block_id {
            id::PAUSE => {
                // 2 bytes: pause in ms LE
                if pos + 2 > data.len() {
                    return Err(Error::Tzx("Truncated PAUSE block".into()));
                }
                let ms = u16::from_le_bytes([data[pos], data[pos + 1]]);
                pos += 2;
                TzxBlock::Pause(ms)
            }

            id::TURBO_LOADING => {
                // 18 bytes header after ID
                if pos + 18 > data.len() {
                    return Err(Error::Tzx("Truncated TURBO header".into()));
                }
                let h = &data[pos..pos + 18];
                let pilot_pulse = u16::from_le_bytes([h[0], h[1]]);
                let sync1 = u16::from_le_bytes([h[2], h[3]]);
                let sync2 = u16::from_le_bytes([h[4], h[5]]);
                let zero = u16::from_le_bytes([h[6], h[7]]);
                let one = u16::from_le_bytes([h[8], h[9]]);
                let pilot_pulses = u16::from_le_bytes([h[10], h[11]]);
                let used_bits = h[12];
                let pause_ms = u16::from_le_bytes([h[13], h[14]]);
                let data_len = u24_le(&h[15..18]) as usize;
                pos += 18;

                if pos + data_len > data.len() {
                    return Err(Error::Tzx("Truncated TURBO data".into()));
                }
                let block_data = data[pos..pos + data_len].to_vec();
                pos += data_len;

                TzxBlock::Turbo(TurboBlock {
                    header: TurboHeader {
                        pilot_pulse,
                        sync1,
                        sync2,
                        zero,
                        one,
                        pilot_pulses,
                        used_bits_last_byte: used_bits,
                        pause_ms,
                    },
                    data: block_data,
                })
            }

            id::STANDARD_SPEED => {
                // 4 bytes header after ID
                if pos + 4 > data.len() {
                    return Err(Error::Tzx("Truncated STANDARD header".into()));
                }
                let h = &data[pos..pos + 4];
                let pause_ms = u16::from_le_bytes([h[0], h[1]]);
                let data_len = u16::from_le_bytes([h[2], h[3]]) as usize;
                pos += 4;

                if pos + data_len > data.len() {
                    return Err(Error::Tzx("Truncated STANDARD data".into()));
                }
                let block_data = data[pos..pos + data_len].to_vec();
                pos += data_len;

                TzxBlock::Standard(StandardBlock {
                    pause_ms,
                    data: block_data,
                })
            }

            id::PURE_DATA => {
                // 10 bytes header after ID
                if pos + 10 > data.len() {
                    return Err(Error::Tzx("Truncated PURE DATA header".into()));
                }
                let h = &data[pos..pos + 10];
                let zero = u16::from_le_bytes([h[0], h[1]]);
                let one = u16::from_le_bytes([h[2], h[3]]);
                let used_bits = h[4];
                let pause_ms = u16::from_le_bytes([h[5], h[6]]);
                let data_len = u24_le(&h[7..10]) as usize;
                pos += 10;

                if pos + data_len > data.len() {
                    return Err(Error::Tzx("Truncated PURE DATA data".into()));
                }
                let block_data = data[pos..pos + data_len].to_vec();
                pos += data_len;

                TzxBlock::PureData(PureDataBlock {
                    zero,
                    one,
                    used_bits_last_byte: used_bits,
                    pause_ms,
                    data: block_data,
                })
            }

            _ => {
                // Unknown block: try to handle known-size blocks from TZX spec,
                // otherwise store raw bytes until end of file.
                let raw = parse_unknown_block(block_id, data, &mut pos)?;
                TzxBlock::Unknown { id: block_id, raw }
            }
        };

        blocks.push(block);
    }

    Ok(TzxFile {
        version_major,
        version_minor,
        blocks,
    })
}

// ─────────────────────────────────────────────────────────────────────────────

fn u24_le(b: &[u8]) -> u32 {
    b[0] as u32 | (b[1] as u32) << 8 | (b[2] as u32) << 16
}

/// Parse an unknown block by its ID using the TZX spec for size information.
/// Returns the raw bytes (not including the already-consumed ID byte).
fn parse_unknown_block(id: u8, data: &[u8], pos: &mut usize) -> Result<Vec<u8>> {
    let start = *pos;

    let extra: usize = match id {
        0x12 => 4,                    // Pure Tone: pulse_len(2) + count(2)
        0x13 => {                     // Pulse Sequence: count(1) + count*2 bytes
            if *pos >= data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let count = data[*pos] as usize;
            1 + count * 2
        }
        0x15 => {                     // Direct Recording: fixed 8 bytes + data(24-bit len)
            if *pos + 8 > data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let data_len = u24_le(&data[*pos + 5..*pos + 8]) as usize;
            8 + data_len
        }
        0x20 => 2,                    // Pause (handled above, shouldn't reach here)
        0x21 => {                     // Group Start: name_len(1) + name
            if *pos >= data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let l = data[*pos] as usize;
            1 + l
        }
        0x22 => 0,                    // Group End: no data
        0x23 => 2,                    // Jump to Block
        0x24 => 2,                    // Loop Start
        0x25 => 0,                    // Loop End
        0x26 => {                     // Call Sequence
            if *pos + 2 > data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let count = u16::from_le_bytes([data[*pos], data[*pos + 1]]) as usize;
            2 + count * 2
        }
        0x27 => 0,                    // Return from Sequence
        0x28 => {                     // Select Block
            if *pos + 2 > data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let l = u16::from_le_bytes([data[*pos], data[*pos + 1]]) as usize;
            2 + l
        }
        0x30 => {                     // Text Description
            if *pos >= data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let l = data[*pos] as usize;
            1 + l
        }
        0x31 => {                     // Message Block
            if *pos + 2 > data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let l = data[*pos + 1] as usize;
            2 + l
        }
        0x32 => {                     // Archive Info
            if *pos + 2 > data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let l = u16::from_le_bytes([data[*pos], data[*pos + 1]]) as usize;
            2 + l
        }
        0x33 => {                     // Hardware Type
            if *pos >= data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let count = data[*pos] as usize;
            1 + count * 3
        }
        0x35 => {                     // Custom Info: id_str(10) + len(4) + data
            if *pos + 14 > data.len() {
                return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
            }
            let l = u32::from_le_bytes([
                data[*pos + 10], data[*pos + 11],
                data[*pos + 12], data[*pos + 13],
            ]) as usize;
            14 + l
        }
        0x5A => 9,                    // Glue Block
        _ => {
            // Completely unknown: we cannot determine the size, return what's left
            eprintln!("Warning: unknown TZX block 0x{:02X} — consuming rest of file", id);
            let remaining = data.len() - *pos;
            *pos = data.len();
            return Ok(data[start..start + remaining].to_vec());
        }
    };

    if *pos + extra > data.len() {
        return Err(Error::Tzx(format!("Truncated block 0x{:02X}", id)));
    }
    let raw = data[start..*pos + extra].to_vec();
    *pos += extra;
    Ok(raw)
}
