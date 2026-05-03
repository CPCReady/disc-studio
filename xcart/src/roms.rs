// MIT License — Copyright (c) Destroyer 2026.
//
// ROMs are loaded at runtime from a directory.

use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Error, Result};

const ROM_SIZE: usize = 16 * 1024;

pub struct RomSet {
    pub os: Vec<u8>,
    pub basic: Vec<u8>,
    pub amsdos: Vec<u8>,
    pub source_dir: PathBuf,
}

pub fn load(roms_dir: Option<&Path>) -> Result<RomSet> {
    let source_dir = resolve_roms_dir(roms_dir)?;

    let os = read_rom(&source_dir, "os.rom")?;
    let basic = read_rom(&source_dir, "basic.rom")?;
    let amsdos = read_rom(&source_dir, "amsdos.rom")?;

    Ok(RomSet {
        os,
        basic,
        amsdos,
        source_dir,
    })
}

fn resolve_roms_dir(roms_dir: Option<&Path>) -> Result<PathBuf> {
    if let Some(dir) = roms_dir {
        return Ok(dir.to_path_buf());
    }

    if let Ok(env_dir) = std::env::var("XCART_ROMS_DIR") {
        let trimmed = env_dir.trim();
        if !trimmed.is_empty() {
            return Ok(PathBuf::from(trimmed));
        }
    }

    Err(Error::Other(
        "ROM directory not set. Use --roms-dir <path> (or XCART_ROMS_DIR) with os.rom, basic.rom and amsdos.rom".to_string(),
    ))
}

fn read_rom(dir: &Path, name: &str) -> Result<Vec<u8>> {
    let path = dir.join(name);
    let data = fs::read(&path).map_err(|e| {
        Error::Other(format!(
            "Missing ROM '{}': {} ({})",
            name,
            path.display(),
            e
        ))
    })?;

    if data.len() != ROM_SIZE {
        return Err(Error::Other(format!(
            "Invalid ROM size for '{}': {} bytes (expected {} bytes)",
            path.display(),
            data.len(),
            ROM_SIZE
        )));
    }

    Ok(data)
}
