use std::path::Path;

use crate::crc;
use crate::error::{Error, Result};
use crate::timing::{CPC_DATA_CHUNK_SIZE, SYNC_HEADER};
use crate::tzx::{read_file, write_file, TzxBlock};

/// `xcdt rename <input.cdt> <old_name> <new_name> [-o output.cdt]`
///
/// Finds all AMSDOS header blocks whose filename matches `old_name`
/// (case-insensitive), patches the 16-byte name field in-place and
/// recalculates the block checksum/CRC. No audio re-encoding is done.
///
/// If `output` is `None` the CDT is overwritten in place.
pub fn run(input: &Path, old_name: &str, new_name: &str, output: Option<&Path>) -> Result<()> {
    let mut tzx = read_file(input)?;

    let old_upper = old_name.trim().to_ascii_uppercase();
    let new_upper = new_name.trim().to_ascii_uppercase();

    // Build 16-byte name field (space-padded with zeros beyond the text).
    let mut name_field = [0u8; 16];
    let n = new_upper.len().min(16);
    name_field[..n].copy_from_slice(&new_upper.as_bytes()[..n]);

    let mut count = 0usize;

    for block in tzx.blocks.iter_mut() {
        match block {
            // ── Turbo Loading Data Block ──────────────────────────────────────
            // Layout: [sync(1)] [chunk_256(256)] [crc(2)] ... [trailer_4(4)]
            // For a 64-byte tape header the block is exactly 263 bytes.
            TzxBlock::Turbo(b) => {
                if b.data.len() < 1 + CPC_DATA_CHUNK_SIZE + 2 {
                    continue;
                }
                if b.data[0] != SYNC_HEADER {
                    continue;
                }
                // Name is at bytes 1..17 (inside the first 256-byte chunk).
                let current = name_str(&b.data[1..17]);
                if current.to_ascii_uppercase() != old_upper {
                    continue;
                }
                // Patch name field.
                b.data[1..17].copy_from_slice(&name_field);
                // Recompute CRC over the full 256-byte chunk (bytes 1..257).
                let crc_pos = 1 + CPC_DATA_CHUNK_SIZE;
                let crc_bytes = crc::compute_inverted(&b.data[1..crc_pos]);
                b.data[crc_pos] = crc_bytes[0];
                b.data[crc_pos + 1] = crc_bytes[1];
                count += 1;
            }

            // ── Standard Speed Data Block ─────────────────────────────────────
            // Layout: [sync(1)] [data...] [XOR_checksum(1)]
            // For a 64-byte tape header the block is 66 bytes.
            TzxBlock::Standard(b) => {
                if b.data.len() < 66 {
                    continue;
                }
                if b.data[0] != SYNC_HEADER {
                    continue;
                }
                let current = name_str(&b.data[1..17]);
                if current.to_ascii_uppercase() != old_upper {
                    continue;
                }
                // Patch name field.
                b.data[1..17].copy_from_slice(&name_field);
                // Recompute XOR checksum over all bytes except the last.
                let last = b.data.len() - 1;
                let cs = b.data[..last].iter().fold(0u8, |acc, &x| acc ^ x);
                b.data[last] = cs;
                count += 1;
            }

            _ => {}
        }
    }

    if count == 0 {
        return Err(Error::Other(format!(
            "No file named '{}' found on tape",
            old_upper
        )));
    }

    let out_path = output.unwrap_or(input);
    write_file(&tzx, out_path)?;
    eprintln!(
        "Renamed '{}' → '{}' ({} header block{}) → {}",
        old_upper,
        new_upper,
        count,
        if count == 1 { "" } else { "s" },
        out_path.display()
    );

    Ok(())
}

/// Returns the null/space-trimmed ASCII name from a raw byte slice.
fn name_str(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).trim().to_string()
}
