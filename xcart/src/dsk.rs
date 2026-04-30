// MIT License — Copyright (c) Destroyer 2026.
//
// DSK disk image parser — supports both Standard (MV-CPCEMU) and Extended formats.
// Reference: http://www.cpcwiki.eu/index.php/Format:DSK_disk_image_file_format

use std::collections::BTreeMap;
use std::fmt;
use std::path::Path;

use crate::error::{Error, Result};

// ─────────────────────────────────────────────
// Public types
// ─────────────────────────────────────────────

/// A single sector within a track.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Sector {
    pub track: u8,
    pub side: u8,
    pub identifier: u8,
    pub size: usize,
    pub data: Vec<u8>,
}

/// A track containing one or more sectors, keyed by sector identifier.
/// `BTreeMap` guarantees ascending-order iteration by sector identifier,
/// matching the Python's `sorted(self.sectors)` behaviour.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Track {
    pub number: u8,
    pub side: u8,
    pub sector_size: usize,
    pub sectors: BTreeMap<u8, Sector>,
}

impl Track {
    /// Iterates sectors in ascending identifier order.
    pub fn sorted_sectors(&self) -> impl Iterator<Item = &Sector> {
        self.sectors.values()
    }
}

/// Disk image format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DskFormat {
    /// Classic MV-CPCEMU format (fixed track sizes).
    Standard,
    /// Extended DSK format (variable track sizes).
    Extended,
}

impl fmt::Display for DskFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DskFormat::Standard => write!(f, "Standard (MV-CPCEMU)"),
            DskFormat::Extended => write!(f, "Extended"),
        }
    }
}

/// Parsed DSK disk image.
#[derive(Debug)]
pub struct DskFile {
    pub format: DskFormat,
    pub creator: String,
    pub tracks_count: u8,
    pub sides_count: u8,
    pub tracks: Vec<Track>,
}

impl DskFile {
    /// Open and parse a DSK file from disk.
    pub fn open(path: &Path) -> Result<Self> {
        let data = std::fs::read(path)?;
        Self::parse(&data)
    }

    /// Parse a DSK image from a raw byte slice (pure, no I/O).
    pub fn parse(data: &[u8]) -> Result<Self> {
        if data.len() < 0x100 {
            return Err(Error::Dsk("File too small to be a DSK image".into()));
        }

        // ── Format detection ───────────────────────────────────────────────
        let format = if data.starts_with(b"MV - CPC") {
            DskFormat::Standard
        } else if data.starts_with(b"EXTENDED") || data.starts_with(b"EXT") {
            DskFormat::Extended
        } else {
            return Err(Error::Dsk(
                "Unsupported DSK format (expected 'MV - CPC...' or 'EXTENDED...')".into(),
            ));
        };

        // ── Header fields ──────────────────────────────────────────────────
        let creator = String::from_utf8_lossy(&data[0x22..0x30])
            .trim_end_matches('\0')
            .trim()
            .to_string();

        let tracks_count = data[0x30];
        let sides_count = data[0x31];
        let num_entries = tracks_count as usize * sides_count as usize;

        if num_entries == 0 {
            return Err(Error::Dsk("DSK reports 0 tracks or 0 sides".into()));
        }

        // ── Track size table ───────────────────────────────────────────────
        let track_sizes: Vec<usize> = match format {
            DskFormat::Standard => {
                let track_size =
                    u16::from_le_bytes([data[0x32], data[0x33]]) as usize;
                vec![track_size; num_entries]
            }
            DskFormat::Extended => {
                if data.len() < 0x34 + num_entries {
                    return Err(Error::Dsk(
                        "File truncated: extended track-size table incomplete".into(),
                    ));
                }
                data[0x34..0x34 + num_entries]
                    .iter()
                    .map(|&b| (b as usize) * 0x100)
                    .collect()
            }
        };

        // ── Track data ─────────────────────────────────────────────────────
        let mut tracks: Vec<Track> = Vec::new();
        let mut offset: usize = 0x100;

        for track_idx in 0..num_entries {
            let declared_size = track_sizes[track_idx];
            if declared_size == 0 {
                // Extended DSK may have empty track entries — skip silently.
                continue;
            }

            let expected_track_id = (track_idx / sides_count as usize) as u8;
            let expected_side_id = (track_idx % sides_count as usize) as u8;

            if offset + declared_size > data.len() {
                return Err(Error::Dsk(format!(
                    "Track {} at offset 0x{:X} extends beyond end of file",
                    track_idx, offset
                )));
            }

            let track_slice = &data[offset..offset + declared_size];
            let track = parse_track(
                track_slice,
                track_idx,
                sides_count as usize,
                expected_track_id,
                expected_side_id,
            )?;

            tracks.push(track);
            offset += declared_size;
        }

        Ok(DskFile {
            format,
            creator,
            tracks_count,
            sides_count,
            tracks,
        })
    }

    /// Returns the minimum sector identifier of the first track.
    /// Used to patch the AMSDOS ROM for standard-format disks.
    pub fn min_sector_id(&self) -> u8 {
        self.tracks
            .first()
            .and_then(|t| t.sectors.keys().next().copied())
            .unwrap_or(0xC1)
    }

    /// Total number of sectors across all tracks.
    pub fn total_sectors(&self) -> usize {
        self.tracks.iter().map(|t| t.sectors.len()).sum()
    }

    /// Concatenates all sector data in track-then-sector-identifier order.
    /// This is the raw payload that gets packed into CPR data chunks.
    pub fn collect_sector_data(&self) -> Vec<u8> {
        let mut out =
            Vec::with_capacity(self.total_sectors() * 512);
        for track in &self.tracks {
            for sector in track.sorted_sectors() {
                out.extend_from_slice(&sector.data);
            }
        }
        out
    }
}

impl fmt::Display for DskFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} sided {} disk: {} tracks/side, {} sectors total",
            self.sides_count,
            self.format,
            self.tracks_count,
            self.total_sectors()
        )
    }
}

// ─────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────

/// Parse a single track from its raw byte slice.
fn parse_track(
    data: &[u8],
    track_idx: usize,
    _sides_count: usize,
    expected_track: u8,
    expected_side: u8,
) -> Result<Track> {
    if data.len() < 0x100 {
        return Err(Error::Dsk(format!(
            "Track {} data too small ({} bytes)",
            track_idx,
            data.len()
        )));
    }

    // Magic
    if &data[0x00..0x0A] != b"Track-Info" {
        return Err(Error::Dsk(format!(
            "Track {} missing 'Track-Info' magic",
            track_idx
        )));
    }

    let number = data[0x10];
    let side = data[0x11];
    let sector_size = 0x80usize << data[0x14];
    let sectors_count = data[0x15] as usize;

    // Validate track/side numbers
    if number != expected_track {
        return Err(Error::Dsk(format!(
            "Track {}: found track number {}, expected {}",
            track_idx, number, expected_track
        )));
    }
    if side != expected_side {
        return Err(Error::Dsk(format!(
            "Track {}: found side {}, expected {}",
            track_idx, side, expected_side
        )));
    }

    // ── Parse sector entries ───────────────────────────────────────────────
    let mut sectors: BTreeMap<u8, Sector> = BTreeMap::new();

    for i in 0..sectors_count {
        let h = 0x18 + i * 8; // sector entry in track header

        if h + 8 > data.len() {
            return Err(Error::Dsk(format!(
                "Track {} sector {} entry at 0x{:X} is out of bounds",
                track_idx, i, h
            )));
        }

        let s_track = data[h];
        let s_side = data[h + 1];
        let identifier = data[h + 2];
        let size_code = data[h + 3];
        let declared_size = 0x80usize << size_code;
        let length = data[h + 6] as usize + data[h + 7] as usize * 0x100;

        // Validate: length (from bytes [6-7]) must match track's sector_size (if non-zero)
        if length != 0 && length != sector_size {
            return Err(Error::Dsk(format!(
                "Track {} sector {} (id=0x{:02X}): declared length {} \
                 does not match track sector size {}",
                track_idx, i, identifier, length, sector_size
            )));
        }

        // Validate: size code must agree with track-level sector size
        if declared_size != sector_size {
            return Err(Error::Dsk(format!(
                "Track {} sector {} (id=0x{:02X}): size code gives {} \
                 but track sector size is {}",
                track_idx, i, identifier, declared_size, sector_size
            )));
        }

        // Sector data immediately follows the 0x100-byte track header
        let data_offset = 0x100 + i * sector_size;
        if data_offset + sector_size > data.len() {
            return Err(Error::Dsk(format!(
                "Track {} sector {} (id=0x{:02X}): data at 0x{:X} is out of bounds",
                track_idx, i, identifier, data_offset
            )));
        }

        sectors.insert(
            identifier,
            Sector {
                track: s_track,
                side: s_side,
                identifier,
                size: sector_size,
                data: data[data_offset..data_offset + sector_size].to_vec(),
            },
        );
    }

    Ok(Track {
        number,
        side,
        sector_size,
        sectors,
    })
}
