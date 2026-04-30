#![allow(dead_code)]
/// T-states per second in the TZX/Spectrum reference clock.
pub const TZX_T_STATES: i64 = 3_500_000;

/// CPC NOPs per frame (50 Hz).
const CPC_NOPS_PER_FRAME: i64 = 19_968;
/// CPC NOPs per second.
const CPC_NOPS_PER_SECOND: i64 = CPC_NOPS_PER_FRAME * 50;
/// CPC T-states per second (4 T-states per NOP).
const CPC_T_STATES: i64 = CPC_NOPS_PER_SECOND * 4; // 3,993,600

/// Compile-time conversion factor (fixed-point 8.8) from CPC T-states to TZX T-states.
/// Formula: (TZX_T_STATES << 8) / (CPC_T_STATES >> 8)
const T_STATE_CONVERSION_FACTOR: i64 =
    (TZX_T_STATES << 8) / (CPC_T_STATES >> 8); // = 57435

/// Pilot tone: 2048 waves = 4096 pulses (one wave = two pulses).
pub const CPC_PILOT_TONE_NUM_WAVES: usize = 2048;
pub const CPC_PILOT_TONE_NUM_PULSES: u16 = (CPC_PILOT_TONE_NUM_WAVES * 2) as u16;

/// Pause after a data block (ms).
pub const CPC_PAUSE_AFTER_BLOCK_MS: u16 = 2500;
/// Pause between the tape header block and the following data block (ms).
pub const CPC_PAUSE_AFTER_HEADER_MS: u16 = 10;

/// Sync byte used in CPC tape headers (header blocks).
pub const SYNC_HEADER: u8 = 0x2C;
/// Sync byte used in CPC tape data blocks.
pub const SYNC_DATA: u8 = 0x16;
/// Sync byte used for Spectrum-compatible blocks.
pub const SYNC_SPECTRUM: u8 = 0xFF;

/// Size of a CPC tape data chunk (bytes, before CRC).
pub const CPC_DATA_CHUNK_SIZE: usize = 256;
/// Standard CPC tape block size (bytes).
pub const CPC_DATA_BLOCK_SIZE: usize = 2048;

/// Baud rate defaults and limits.
pub const DEFAULT_BAUD: u32 = 2000;
pub const MIN_BAUD: u32 = 1000;
pub const MAX_BAUD: u32 = 6000;

/// Timing parameters for a given baud rate, expressed in TZX T-states.
#[derive(Debug, Clone, Copy)]
pub struct TimingParams {
    /// Pilot (= ONE) pulse length in TZX T-states.
    pub pilot_pulse: u16,
    /// SYNC first pulse length (= zero pulse).
    pub sync1: u16,
    /// SYNC second pulse length (= zero pulse).
    pub sync2: u16,
    /// Zero bit pulse length.
    pub zero: u16,
    /// One bit pulse length (= 2 × zero).
    pub one: u16,
    /// Number of pilot pulses (= 2048 waves × 2).
    pub pilot_pulses: u16,
}

impl TimingParams {
    /// Compute timing parameters for the given baud rate.
    /// Matches the exact integer arithmetic used by the original 2cdt C code.
    pub fn for_baud(baud: u32) -> Self {
        // equation from CPC firmware guide:
        // Average baud rate = 333333 / half_zero_length
        let zero_us: i64 = 333_333 / baud as i64;
        let zero_cpc: i64 = zero_us << 2; // multiply by 4 (T-states per µs)

        // Convert CPC T-states → TZX T-states using the fixed-point factor
        let zero_tzx = (zero_cpc * (T_STATE_CONVERSION_FACTOR >> 8)) >> 8;
        let one_tzx = zero_tzx << 1; // one bit = twice zero bit

        TimingParams {
            pilot_pulse: one_tzx as u16,  // pilot = ONE pulse on CPC
            sync1: zero_tzx as u16,
            sync2: zero_tzx as u16,
            zero: zero_tzx as u16,
            one: one_tzx as u16,
            pilot_pulses: CPC_PILOT_TONE_NUM_PULSES,
        }
    }
}
