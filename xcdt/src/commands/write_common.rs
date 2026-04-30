// Shared logic for `new` and `save` commands.
// This module is not exposed in commands/mod.rs directly;
// it is used by new.rs and save.rs.

use std::path::Path;

use crate::amsdos::{has_amsdos_header, AMSDOS_DISK_HEADER_SIZE, TAPE_HEADER_SIZE};
use crate::crc;
use crate::encoder::encode_pure_data;
use crate::error::Result;
use crate::timing::*;
use crate::tzx::blocks::*;
use crate::tzx::{TzxFile, TzxBlock};

// ── Options passed to the core write function ─────────────────────────────────

/// Method used to structure the tape data blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataMethod {
    /// Standard CPC method: header + 2 KB data blocks (default).
    Blocks,
    /// Headerless: one continuous data block, no AMSDOS tape header.
    Headerless,
    /// Spectrum standard speed.
    Spectrum,
}

/// TZX block encoding type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    /// Turbo Loading (ID 0x11) — default.
    Turbo,
    /// Pure Data (ID 0x14).
    PureData,
    /// Standard Speed (ID 0x10).
    Standard,
}

/// Options for the `new` / `save` commands.
#[derive(Debug, Clone)]
pub struct WriteOptions {
    pub baud: u32,
    pub block_type: BlockType,
    pub method: DataMethod,
    pub tape_name: Option<String>,
    pub load_address: Option<u16>,
    pub exec_address: Option<u16>,
    pub file_type: Option<u8>,
    /// Initial pause in ms before the first block (used when creating new CDT).
    pub initial_pause_ms: u16,
    /// Add an extra 1 ms pause before the initial pause (for buggy emulators).
    pub buggy_emu_pause: bool,
}

impl Default for WriteOptions {
    fn default() -> Self {
        WriteOptions {
            baud: DEFAULT_BAUD,
            block_type: BlockType::Turbo,
            method: DataMethod::Blocks,
            tape_name: None,
            load_address: None,
            exec_address: None,
            file_type: None,
            initial_pause_ms: 3000,
            buggy_emu_pause: false,
        }
    }
}

// ── Core builder ──────────────────────────────────────────────────────────────

/// Build a `TzxFile` from a source file and options. Used by both `new` and `save`.
///
/// `include_initial_pause` should be `true` for `new` (blank CDT) and `false`
/// for `save` (append to existing).
pub fn build_tzx(
    source: &Path,
    opts: &WriteOptions,
    include_initial_pause: bool,
) -> Result<TzxFile> {
    let raw_data = std::fs::read(source)?;
    let mut tzx = TzxFile::new();
    let timing = TimingParams::for_baud(opts.baud);

    // ── Detect AMSDOS header ──────────────────────────────────────────────────
    let (file_offset, file_data) = if raw_data.len() >= AMSDOS_DISK_HEADER_SIZE
        && has_amsdos_header(&raw_data)
    {
        // File has a valid AMSDOS header — extract metadata, skip 128-byte header
        (AMSDOS_DISK_HEADER_SIZE, &raw_data[AMSDOS_DISK_HEADER_SIZE..])
    } else {
        (0, raw_data.as_slice())
    };

    // Build base tape header from AMSDOS data (if available) or from options defaults
    let mut tape_header = [0u8; TAPE_HEADER_SIZE];

    if file_offset > 0 {
        // Copy from AMSDOS header
        tape_header.copy_from_slice(&raw_data[..TAPE_HEADER_SIZE]);
        // Apply overrides
        if let Some(exec) = opts.exec_address {
            tape_header[26] = exec as u8;
            tape_header[27] = (exec >> 8) as u8;
        }
        if let Some(load) = opts.load_address {
            tape_header[21] = load as u8;
            tape_header[22] = (load >> 8) as u8;
        }
        if let Some(ft) = opts.file_type {
            tape_header[18] = ft;
        }
    } else {
        // No AMSDOS header — use option defaults
        let exec = opts.exec_address.unwrap_or(0x1000);
        let load = opts.load_address.unwrap_or(0x1000);
        let ft = opts.file_type.unwrap_or(crate::amsdos::file_type::BINARY);
        tape_header[18] = ft;
        tape_header[19] = file_data.len() as u8;
        tape_header[20] = (file_data.len() >> 8) as u8;
        tape_header[21] = load as u8;
        tape_header[22] = (load >> 8) as u8;
        tape_header[24] = file_data.len() as u8;
        tape_header[25] = (file_data.len() >> 8) as u8;
        tape_header[26] = exec as u8;
        tape_header[27] = (exec >> 8) as u8;
    }

    // Apply tape filename
    if let Some(ref name) = opts.tape_name {
        let n = name.len().min(16);
        for i in 0..n {
            tape_header[i] = name.as_bytes()[i].to_ascii_uppercase();
        }
        // zero out rest of name field
        for i in n..16 {
            tape_header[i] = 0;
        }
    }

    // Set logical length (total file size)
    let logical_len = file_data.len() as u16;
    tape_header[24] = logical_len as u8;
    tape_header[25] = (logical_len >> 8) as u8;

    // ── Initial pause (new CDT only) ──────────────────────────────────────────
    if include_initial_pause {
        if opts.buggy_emu_pause {
            tzx.push(TzxBlock::Pause(1));
        }
        tzx.push(TzxBlock::Pause(opts.initial_pause_ms));
    }

    // ── Write data per method ─────────────────────────────────────────────────
    if opts.method == DataMethod::Spectrum {
        // Spectrum standard speed block: single block, sync=0xFF, no CPC tape header
        let block = make_standard_block(SYNC_SPECTRUM, file_data, 1000);
        tzx.push(block);
        return Ok(tzx);
    }

    // CPC methods: BLOCKS or HEADERLESS
    let mut remaining = file_data;
    let mut block_index: u8 = 1;
    let mut first_block = true;
    let mut block_location = u16::from_le_bytes([tape_header[21], tape_header[22]]);

    loop {
        // Determine this block's size
        let (tape_block_size, last_block) = if opts.method == DataMethod::Headerless {
            (remaining.len(), true)
        } else {
            if remaining.len() > CPC_DATA_BLOCK_SIZE {
                (CPC_DATA_BLOCK_SIZE, false)
            } else {
                (remaining.len(), true)
            }
        };

        // Update tape header fields for this block
        tape_header[16] = block_index;
        tape_header[17] = if last_block { 0xFF } else { 0x00 };
        tape_header[23] = if first_block { 0xFF } else { 0x00 };
        tape_header[19] = tape_block_size as u8;
        tape_header[20] = (tape_block_size >> 8) as u8;
        tape_header[21] = block_location as u8;
        tape_header[22] = (block_location >> 8) as u8;

        // Write CPC tape header block (not for Headerless)
        if opts.method != DataMethod::Headerless {
            let hdr_block = make_data_block(
                opts.block_type,
                &timing,
                SYNC_HEADER,
                &tape_header,
                CPC_PAUSE_AFTER_HEADER_MS,
            );
            tzx.push(hdr_block);
        }

        // Write data block
        let data_block = make_data_block(
            opts.block_type,
            &timing,
            SYNC_DATA,
            &remaining[..tape_block_size],
            CPC_PAUSE_AFTER_BLOCK_MS,
        );
        tzx.push(data_block);

        block_location = block_location.wrapping_add(tape_block_size as u16);
        block_index += 1;
        first_block = false;
        remaining = &remaining[tape_block_size..];

        if remaining.is_empty() {
            break;
        }
    }

    Ok(tzx)
}

// ── Block constructors ────────────────────────────────────────────────────────

/// Public entry point for building a block of any type. Used by `new`, `save`
/// and `convert`.
pub fn make_block(
    bt: BlockType,
    timing: &TimingParams,
    sync: u8,
    data: &[u8],
    pause_ms: u16,
) -> TzxBlock {
    match bt {
        BlockType::Turbo => make_turbo_block(timing, sync, data, pause_ms),
        BlockType::PureData => make_pure_data_block(timing, sync, data, pause_ms),
        BlockType::Standard => make_standard_block(sync, data, pause_ms),
    }
}

fn make_data_block(
    bt: BlockType,
    timing: &TimingParams,
    sync: u8,
    data: &[u8],
    pause_ms: u16,
) -> TzxBlock {
    make_block(bt, timing, sync, data, pause_ms)
}

/// Build a Turbo Loading Data Block.
///
/// Layout: [sync_byte] [N × (256 bytes + 2 CRC)] [4 × 0xFF trailer]
fn make_turbo_block(timing: &TimingParams, sync: u8, data: &[u8], pause_ms: u16) -> TzxBlock {
    let num_chunks = (data.len() + CPC_DATA_CHUNK_SIZE - 1) / CPC_DATA_CHUNK_SIZE;

    // Size: 1 sync + N*(256+2 CRC) + 4 trailer
    let block_size = 1 + num_chunks * (CPC_DATA_CHUNK_SIZE + 2) + 4;
    let mut payload = vec![0u8; block_size];

    payload[0] = sync;
    let mut pos = 1usize;
    let mut remaining = data;

    for _ in 0..num_chunks {
        let chunk_len = remaining.len().min(CPC_DATA_CHUNK_SIZE);
        // Copy data (zero-pad to 256)
        payload[pos..pos + chunk_len].copy_from_slice(&remaining[..chunk_len]);
        // Already zeroed (vec! initializes to 0)

        // Compute CRC over the full 256-byte chunk (including padding)
        let crc_bytes = crc::compute_inverted(&payload[pos..pos + CPC_DATA_CHUNK_SIZE]);
        pos += CPC_DATA_CHUNK_SIZE;
        payload[pos] = crc_bytes[0];
        payload[pos + 1] = crc_bytes[1];
        pos += 2;

        remaining = &remaining[chunk_len..];
    }

    // Trailer: 4 × 0xFF
    payload[pos..pos + 4].fill(0xFF);

    TzxBlock::Turbo(TurboBlock {
        header: TurboHeader {
            pilot_pulse: timing.pilot_pulse,
            sync1: timing.sync1,
            sync2: timing.sync2,
            zero: timing.zero,
            one: timing.one,
            pilot_pulses: timing.pilot_pulses,
            used_bits_last_byte: 8,
            pause_ms,
        },
        data: payload,
    })
}

/// Build a Pure Data Block (bitstream encoded).
fn make_pure_data_block(timing: &TimingParams, sync: u8, data: &[u8], pause_ms: u16) -> TzxBlock {
    let bitstream = encode_pure_data(sync, data);
    TzxBlock::PureData(PureDataBlock {
        zero: timing.zero,
        one: timing.one,
        used_bits_last_byte: 8,
        pause_ms,
        data: bitstream,
    })
}

/// Build a Standard Speed Data Block.
///
/// Layout: [sync_byte] [data...] [XOR checksum of sync+data]
fn make_standard_block(sync: u8, data: &[u8], pause_ms: u16) -> TzxBlock {
    let mut payload = Vec::with_capacity(data.len() + 2);
    let mut checksum = sync;
    payload.push(sync);
    for &b in data {
        checksum ^= b;
        payload.push(b);
    }
    payload.push(checksum);

    TzxBlock::Standard(StandardBlock {
        pause_ms,
        data: payload,
    })
}
