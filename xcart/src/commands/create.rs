// MIT License — Copyright (c) Destroyer 2026.
//
// `xcart create` — convert a DSK disk image into a GX-4000 CPR cartridge.
//
// CPR chunk layout:
//   cb00  OS ROM    (embedded)
//   cb01  BASIC ROM (embedded)
//   cb02  AMSDOS ROM (embedded, patched in-memory)
//   cb03… DSK sector data, packed in 16 KB blocks

use std::path::Path;

use crate::cpr::{CprWriter, CHUNK_SIZE};
use crate::dsk::{DskFile, DskFormat};
use crate::error::{Error, Result};
use crate::roms;

// ── AMSDOS ROM patch offsets ──────────────────────────────────────────────────

/// Byte that enables/disables the autostart command (0 = disabled).
const CMD_FLAG_OFFSET: usize = 0x1C03;

/// 16-byte field containing the null-padded BASIC autostart command string.
const CMD_DATA_OFFSET: usize = 0x1C04;
const CMD_DATA_LEN: usize = 16;

/// Byte patched with the disk's minimum sector identifier (standard disks)
/// or with 0x41 (extended disks).
const SECTOR_ID_OFFSET: usize = 0x056D;

/// Offset of the 22-byte extended-format patch block inside AMSDOS ROM.
const EXT_PATCH_OFFSET: usize = 0x0A43;

/// 22-byte patch applied to AMSDOS ROM when the source disk is in Extended format.
const EXTENDED_PATCH: [u8; 22] = [
    0x24, 0x00, 0x03, 0x07, 0x00, 0xFE, 0x00, 0x3F, 0x00, 0xC0, 0x00,
    0x10, 0x00, 0x00, 0x00, 0xC1, 0x09, 0x2A, 0x52, 0xE5, 0x02, 0x04,
];

// ─────────────────────────────────────────────────────────────────────────────

/// Create a CPR cartridge from `input` DSK and write it to `output`.
///
/// # Arguments
/// * `command` — optional BASIC autostart string (max 16 chars),
///               e.g. `run"disc"` or `|cpm`.
pub fn run(input: &Path, output: &Path, command: Option<&str>) -> Result<()> {
    // Validate command length up front
    if let Some(cmd) = command {
        if cmd.len() > CMD_DATA_LEN {
            return Err(Error::Other(format!(
                "BASIC autostart command must be ≤ {} characters (got {}): '{}'",
                CMD_DATA_LEN,
                cmd.len(),
                cmd
            )));
        }
    }

    // ── Parse source DSK ──────────────────────────────────────────────────
    let dsk = DskFile::open(input)?;
    eprintln!("  Reading : {}", dsk);

    let sector_data = dsk.collect_sector_data();
    let data_chunks: Vec<&[u8]> = sector_data.chunks(CHUNK_SIZE).collect();
    let total_chunks = 3 + data_chunks.len(); // os + basic + amsdos + data

    if total_chunks > crate::cpr::MAX_CHUNKS {
        return Err(Error::Other(format!(
            "Disk is too large: would require {} chunks (max {})",
            total_chunks,
            crate::cpr::MAX_CHUNKS
        )));
    }

    // ── Build CPR ─────────────────────────────────────────────────────────
    let mut writer = CprWriter::new();

    // cb00 — OS ROM
    writer.add_chunk(roms::OS_ROM.to_vec())?;
    eprintln!("  cb00    : OS ROM ({} bytes)", roms::OS_ROM.len());

    // cb01 — BASIC ROM
    writer.add_chunk(roms::BASIC_ROM.to_vec())?;
    eprintln!("  cb01    : BASIC ROM ({} bytes)", roms::BASIC_ROM.len());

    // cb02 — AMSDOS ROM (patched)
    let amsdos = patch_amsdos(roms::AMSDOS_ROM, &dsk, command)?;
    writer.add_chunk(amsdos)?;
    eprintln!("  cb02    : AMSDOS ROM ({} bytes, patched)", roms::AMSDOS_ROM.len());

    // cb03… — DSK sector data
    for (i, chunk) in data_chunks.iter().enumerate() {
        writer.add_chunk(chunk.to_vec())?;
        eprintln!(
            "  cb{:02}    : data chunk {} ({} bytes)",
            3 + i,
            i,
            chunk.len()
        );
    }

    // ── Write output ──────────────────────────────────────────────────────
    writer.write(output)?;

    let file_size = std::fs::metadata(output).map(|m| m.len()).unwrap_or(0);
    eprintln!(
        "  Created : {} ({} chunks, {} bytes)",
        output.display(),
        writer.chunk_count(),
        file_size
    );

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────

/// Apply the required in-memory patches to a copy of the AMSDOS ROM.
fn patch_amsdos(rom: &[u8], dsk: &DskFile, command: Option<&str>) -> Result<Vec<u8>> {
    let mut amsdos = rom.to_vec();

    // ── Patch 1: autostart command ────────────────────────────────────────
    match command {
        Some(cmd) => {
            // Write null-padded command string into the 16-byte field
            let mut buf = [0u8; CMD_DATA_LEN];
            buf[..cmd.len()].copy_from_slice(cmd.as_bytes());
            amsdos[CMD_DATA_OFFSET..CMD_DATA_OFFSET + CMD_DATA_LEN]
                .copy_from_slice(&buf);
            // CMD_FLAG_OFFSET is left as-is (non-zero → autostart enabled)
            eprintln!("  Patch   : autostart command = {:?}", cmd);
        }
        None => {
            // Disable autostart by zeroing the flag byte
            amsdos[CMD_FLAG_OFFSET] = 0;
            eprintln!("  Patch   : autostart disabled");
        }
    }

    // ── Patch 2: disk format ──────────────────────────────────────────────
    match dsk.format {
        DskFormat::Extended => {
            amsdos[EXT_PATCH_OFFSET..EXT_PATCH_OFFSET + EXTENDED_PATCH.len()]
                .copy_from_slice(&EXTENDED_PATCH);
            amsdos[SECTOR_ID_OFFSET] = 0x41;
            eprintln!("  Patch   : extended DSK format");
        }
        DskFormat::Standard => {
            let min_id = dsk.min_sector_id();
            amsdos[SECTOR_ID_OFFSET] = min_id;
            eprintln!("  Patch   : standard DSK, min sector id = 0x{:02X}", min_id);
        }
    }

    Ok(amsdos)
}
