use std::io::Write;
use std::path::Path;

use crate::error::Result;
use super::blocks::*;
use super::TzxFile;
use super::reader::TZX_MAGIC;

/// Write a `TzxFile` to `path`, creating or truncating the file.
pub fn write_file(tzx: &TzxFile, path: &Path) -> Result<()> {
    let mut buf = Vec::new();
    serialize(tzx, &mut buf)?;
    std::fs::write(path, &buf)?;
    Ok(())
}

/// Append a `TzxFile`'s blocks to an existing CDT file (without re-writing the
/// TZX header).  If the file does not exist it is created as a full CDT.
pub fn append_file(tzx: &TzxFile, path: &Path) -> Result<()> {
    if path.exists() {
        // Open for appending and write only blocks (no header).
        let mut f = std::fs::OpenOptions::new().append(true).open(path)?;
        let mut buf = Vec::new();
        for block in &tzx.blocks {
            serialize_block(block, &mut buf)?;
        }
        f.write_all(&buf)?;
    } else {
        // File doesn't exist yet — create a complete CDT.
        write_file(tzx, path)?;
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────

/// Serialize a complete TZX/CDT file (header + blocks) into a byte buffer.
pub fn serialize(tzx: &TzxFile, buf: &mut Vec<u8>) -> Result<()> {
    // Magic + version
    buf.extend_from_slice(TZX_MAGIC);
    buf.push(tzx.version_major);
    buf.push(tzx.version_minor);
    for block in &tzx.blocks {
        serialize_block(block, buf)?;
    }
    Ok(())
}

fn serialize_block(block: &TzxBlock, buf: &mut Vec<u8>) -> Result<()> {
    match block {
        TzxBlock::Pause(ms) => {
            buf.push(id::PAUSE);
            buf.extend_from_slice(&ms.to_le_bytes());
        }

        TzxBlock::Turbo(b) => {
            buf.push(id::TURBO_LOADING);
            let h = &b.header;
            buf.extend_from_slice(&h.pilot_pulse.to_le_bytes());
            buf.extend_from_slice(&h.sync1.to_le_bytes());
            buf.extend_from_slice(&h.sync2.to_le_bytes());
            buf.extend_from_slice(&h.zero.to_le_bytes());
            buf.extend_from_slice(&h.one.to_le_bytes());
            buf.extend_from_slice(&h.pilot_pulses.to_le_bytes());
            buf.push(h.used_bits_last_byte);
            buf.extend_from_slice(&h.pause_ms.to_le_bytes());
            // 24-bit LE data length
            let len = b.data.len() as u32;
            buf.push(len as u8);
            buf.push((len >> 8) as u8);
            buf.push((len >> 16) as u8);
            buf.extend_from_slice(&b.data);
        }

        TzxBlock::Standard(b) => {
            buf.push(id::STANDARD_SPEED);
            buf.extend_from_slice(&b.pause_ms.to_le_bytes());
            let len = b.data.len() as u16;
            buf.extend_from_slice(&len.to_le_bytes());
            buf.extend_from_slice(&b.data);
        }

        TzxBlock::PureData(b) => {
            buf.push(id::PURE_DATA);
            buf.extend_from_slice(&b.zero.to_le_bytes());
            buf.extend_from_slice(&b.one.to_le_bytes());
            buf.push(b.used_bits_last_byte);
            buf.extend_from_slice(&b.pause_ms.to_le_bytes());
            // 24-bit LE data length
            let len = b.data.len() as u32;
            buf.push(len as u8);
            buf.push((len >> 8) as u8);
            buf.push((len >> 16) as u8);
            buf.extend_from_slice(&b.data);
        }

        TzxBlock::Unknown { id, raw } => {
            buf.push(*id);
            buf.extend_from_slice(raw);
        }
    }
    Ok(())
}
