/// TZX/CDT block type IDs.
pub mod id {
    pub const STANDARD_SPEED: u8 = 0x10;
    pub const TURBO_LOADING: u8 = 0x11;
    pub const PURE_DATA: u8 = 0x14;
    pub const PAUSE: u8 = 0x20;
}

// ── Turbo Loading Data Block (0x11) ──────────────────────────────────────────

/// Header parameters for a Turbo Loading Data Block.
#[derive(Debug, Clone)]
pub struct TurboHeader {
    pub pilot_pulse: u16,
    pub sync1: u16,
    pub sync2: u16,
    pub zero: u16,
    pub one: u16,
    pub pilot_pulses: u16,
    pub used_bits_last_byte: u8,
    pub pause_ms: u16,
}

/// A Turbo Loading Data Block (ID 0x11).
#[derive(Debug, Clone)]
pub struct TurboBlock {
    pub header: TurboHeader,
    /// Raw block data as stored in the CDT:
    /// [sync_byte][chunk_256][CRC_2]...[trailer_4]
    pub data: Vec<u8>,
}

impl TurboBlock {
    /// Estimated approximate baud rate from the zero pulse length.
    pub fn approx_baud(&self) -> u32 {
        // TZX T-state → CPC T-state → µs → baud
        // zero_tzx ≈ zero_cpc * T_STATE_FACTOR >> 8; reverse: baud ≈ 333333 / zero_us
        // Approximate: T_zero_cpc = zero_tzx * (CPC_T_STATES / TZX_T_STATES)
        // baud = 333333 / (T_zero_cpc / 4)
        if self.header.zero == 0 {
            return 0;
        }
        let zero_us = (self.header.zero as u64 * 3_993_600) / (3_500_000 * 4);
        if zero_us == 0 {
            return 0;
        }
        (333_333 / zero_us) as u32
    }

    /// Decode the raw data into logical chunks, stripping sync byte and CRCs.
    /// Returns `(sync_byte, chunks_data)` where `chunks_data` is the concatenated
    /// raw chunk bytes (each 256 bytes, last may be shorter).
    pub fn decode(&self) -> Option<(u8, Vec<u8>)> {
        if self.data.is_empty() {
            return None;
        }
        let sync = self.data[0];
        let payload = &self.data[1..];
        // Each chunk: 256 bytes + 2 CRC bytes = 258 bytes
        let mut out = Vec::new();
        let mut pos = 0;
        while pos + 258 <= payload.len() {
            out.extend_from_slice(&payload[pos..pos + 256]);
            pos += 258;
        }
        Some((sync, out))
    }
}

// ── Standard Speed Data Block (0x10) ─────────────────────────────────────────

/// A Standard Speed Data Block (ID 0x10).
#[derive(Debug, Clone)]
pub struct StandardBlock {
    pub pause_ms: u16,
    /// Raw data: [sync_byte][data...][XOR_checksum]
    pub data: Vec<u8>,
}

impl StandardBlock {
    /// Decode: returns `(sync_byte, raw_data)`, stripping sync and checksum.
    pub fn decode(&self) -> Option<(u8, Vec<u8>)> {
        if self.data.len() < 2 {
            return None;
        }
        let sync = self.data[0];
        let raw = self.data[1..self.data.len() - 1].to_vec();
        Some((sync, raw))
    }
}

// ── Pure Data Block (0x14) ───────────────────────────────────────────────────

/// A Pure Data Block (ID 0x14).
#[derive(Debug, Clone)]
pub struct PureDataBlock {
    pub zero: u16,
    pub one: u16,
    pub used_bits_last_byte: u8,
    pub pause_ms: u16,
    /// Raw bitstream data (pilot+sync+chunks+trailer packed MSB-first).
    pub data: Vec<u8>,
}

// ── Union of all block types ──────────────────────────────────────────────────

/// A parsed TZX/CDT block.
#[derive(Debug, Clone)]
pub enum TzxBlock {
    Pause(u16),
    Turbo(TurboBlock),
    Standard(StandardBlock),
    PureData(PureDataBlock),
    /// Any block type not explicitly parsed; raw = [ID, ...header+data bytes]
    Unknown { id: u8, raw: Vec<u8> },
}

impl TzxBlock {
    /// Return the block type ID byte.
    pub fn id(&self) -> u8 {
        match self {
            TzxBlock::Pause(_) => id::PAUSE,
            TzxBlock::Turbo(_) => id::TURBO_LOADING,
            TzxBlock::Standard(_) => id::STANDARD_SPEED,
            TzxBlock::PureData(_) => id::PURE_DATA,
            TzxBlock::Unknown { id, .. } => *id,
        }
    }

    /// Human-readable block type name.
    pub fn type_name(&self) -> &'static str {
        match self {
            TzxBlock::Pause(_) => "PAUSE",
            TzxBlock::Turbo(_) => "TURBO",
            TzxBlock::Standard(_) => "STANDARD",
            TzxBlock::PureData(_) => "PURE DATA",
            TzxBlock::Unknown { .. } => "UNKNOWN",
        }
    }

    /// Data size in bytes (raw payload, not including block header).
    pub fn data_size(&self) -> usize {
        match self {
            TzxBlock::Pause(_) => 0,
            TzxBlock::Turbo(b) => b.data.len(),
            TzxBlock::Standard(b) => b.data.len(),
            TzxBlock::PureData(b) => b.data.len(),
            TzxBlock::Unknown { raw, .. } => raw.len(),
        }
    }

    /// Pause after block in milliseconds (0 if not applicable).
    pub fn pause_ms(&self) -> u16 {
        match self {
            TzxBlock::Pause(ms) => *ms,
            TzxBlock::Turbo(b) => b.header.pause_ms,
            TzxBlock::Standard(b) => b.pause_ms,
            TzxBlock::PureData(b) => b.pause_ms,
            TzxBlock::Unknown { .. } => 0,
        }
    }
}
