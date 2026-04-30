// MIT License — Copyright (c) Destroyer 2026.
//
// CPR cartridge file reader and writer.
// Format: RIFF container with AMS! type tag.
// Reference: http://www.cpcwiki.eu/index.php/Cartridge_format

use std::path::Path;

use crate::error::{Error, Result};

// ─────────────────────────────────────────────
// Constants
// ─────────────────────────────────────────────

/// Maximum number of 16 KB chunks in a CPR file.
pub const MAX_CHUNKS: usize = 32;

/// Fixed size of every chunk payload: 16 KB.
pub const CHUNK_SIZE: usize = 0x4000;

const RIFF_TAG: &[u8; 4] = b"RIFF";
const AMS_TAG: &[u8; 4] = b"AMS!";

/// Byte used to pad chunks shorter than `CHUNK_SIZE`.
const PADDING: u8 = 0xFF;

// ─────────────────────────────────────────────
// CprChunk — represents one parsed chunk
// ─────────────────────────────────────────────

#[derive(Debug)]
pub struct CprChunk {
    pub index: usize,
    pub tag: [u8; 4],
    pub size: u32,
    pub data: Vec<u8>,
}

impl CprChunk {
    pub fn tag_str(&self) -> String {
        String::from_utf8_lossy(&self.tag).to_string()
    }
}

// ─────────────────────────────────────────────
// CprWriter — builds and serialises a CPR file
// ─────────────────────────────────────────────

pub struct CprWriter {
    chunks: Vec<Vec<u8>>,
}

impl CprWriter {
    pub fn new() -> Self {
        CprWriter { chunks: Vec::new() }
    }

    /// Add a data chunk (must be ≤ CHUNK_SIZE bytes).
    /// Padding to CHUNK_SIZE with 0xFF is applied at write time.
    pub fn add_chunk(&mut self, data: Vec<u8>) -> Result<()> {
        if data.len() > CHUNK_SIZE {
            return Err(Error::Cpr(format!(
                "Chunk {} too large: {} bytes (max 0x{:04X})",
                self.chunks.len(),
                data.len(),
                CHUNK_SIZE
            )));
        }
        if self.chunks.len() >= MAX_CHUNKS {
            return Err(Error::Cpr(format!(
                "Too many chunks (max {})",
                MAX_CHUNKS
            )));
        }
        self.chunks.push(data);
        Ok(())
    }

    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }

    /// Serialise the CPR file to disk.
    ///
    /// Layout:
    /// ```
    /// RIFF header  (12 bytes): b"RIFF" + u32_le(payload_size) + b"AMS!"
    /// For each chunk:
    ///   Chunk header (8 bytes):  b"cbNN" + u32_le(CHUNK_SIZE)
    ///   Chunk data   (CHUNK_SIZE bytes): data + 0xFF padding
    /// ```
    pub fn write(&self, path: &Path) -> Result<()> {
        if self.chunks.is_empty() {
            return Err(Error::Cpr("Cannot write CPR with zero chunks".into()));
        }

        // riff_size = len("AMS!") + (CHUNK_SIZE + chunk_header) * N
        let riff_payload: usize =
            4 + (CHUNK_SIZE + 8) * self.chunks.len();

        let total_size = 8 + riff_payload; // 8 = RIFF tag + riff_size field
        let mut out = Vec::with_capacity(total_size);

        // RIFF header
        out.extend_from_slice(RIFF_TAG);
        out.extend_from_slice(&(riff_payload as u32).to_le_bytes());
        out.extend_from_slice(AMS_TAG);

        // Chunks
        for (idx, chunk) in self.chunks.iter().enumerate() {
            let tag = format!("cb{:02}", idx);
            out.extend_from_slice(tag.as_bytes());
            out.extend_from_slice(&(CHUNK_SIZE as u32).to_le_bytes());
            out.extend_from_slice(chunk);
            // Pad to CHUNK_SIZE
            let padding_len = CHUNK_SIZE - chunk.len();
            out.extend(std::iter::repeat(PADDING).take(padding_len));
        }

        debug_assert_eq!(out.len(), total_size);
        std::fs::write(path, &out)?;
        Ok(())
    }
}

// ─────────────────────────────────────────────
// read_cpr — parse and validate an existing CPR
// ─────────────────────────────────────────────

/// Parse a CPR file and return its chunks.
/// Performs full structural validation (tags, sizes, sequential indices).
pub fn read_cpr(path: &Path) -> Result<Vec<CprChunk>> {
    let data = std::fs::read(path)?;
    parse_cpr_bytes(&data)
}

pub fn parse_cpr_bytes(data: &[u8]) -> Result<Vec<CprChunk>> {
    let file_size = data.len();

    if file_size < 12 {
        return Err(Error::Cpr("File too small to be a CPR cartridge".into()));
    }

    // Validate RIFF tag
    if &data[0..4] != RIFF_TAG {
        return Err(Error::Cpr(format!(
            "Not a RIFF file (found {:?})",
            &data[0..4]
        )));
    }

    let riff_size = u32::from_le_bytes([data[4], data[5], data[6], data[7]]) as usize;

    // Validate AMS! type tag
    if &data[8..12] != AMS_TAG {
        return Err(Error::Cpr(format!(
            "Not a CPR cartridge — expected 'AMS!' type tag, found {:?}",
            &data[8..12]
        )));
    }

    // Validate total file size
    if file_size != riff_size + 8 {
        return Err(Error::Cpr(format!(
            "File size mismatch: file is {} bytes but header declares {} (riff_size {} + 8)",
            file_size,
            riff_size + 8,
            riff_size
        )));
    }

    let mut chunks: Vec<CprChunk> = Vec::new();
    let mut offset = 12usize; // skip RIFF header
    let mut chunk_idx = 0usize;

    while offset < file_size {
        // Chunk header: 4-byte tag + 4-byte LE u32 size
        if offset + 8 > file_size {
            return Err(Error::Cpr(format!(
                "Chunk {} header at 0x{:X} is truncated",
                chunk_idx, offset
            )));
        }

        let mut tag = [0u8; 4];
        tag.copy_from_slice(&data[offset..offset + 4]);

        let chunk_size = u32::from_le_bytes([
            data[offset + 4],
            data[offset + 5],
            data[offset + 6],
            data[offset + 7],
        ]);

        // Validate sequential tag "cb00", "cb01", …
        let expected_tag = format!("cb{:02}", chunk_idx);
        if tag != expected_tag.as_bytes() {
            return Err(Error::Cpr(format!(
                "Chunk {} tag is '{}', expected '{}'",
                chunk_idx,
                String::from_utf8_lossy(&tag),
                expected_tag
            )));
        }

        if chunk_size as usize != CHUNK_SIZE {
            eprintln!(
                "Warning: chunk {} size is 0x{:X}, expected 0x{:04X}",
                chunk_idx, chunk_size, CHUNK_SIZE
            );
        }

        let data_start = offset + 8;
        let data_end = data_start + chunk_size as usize;

        if data_end > file_size {
            return Err(Error::Cpr(format!(
                "Chunk {} at 0x{:X} is truncated: need {} bytes, only {} available",
                chunk_idx,
                offset,
                chunk_size,
                file_size - data_start
            )));
        }

        chunks.push(CprChunk {
            index: chunk_idx,
            tag,
            size: chunk_size,
            data: data[data_start..data_end].to_vec(),
        });

        offset = data_end;
        chunk_idx += 1;
    }

    if chunks.is_empty() {
        eprintln!("Warning: CPR file is valid but contains no chunks");
    }

    Ok(chunks)
}
