// MIT License - Copyright (c) 2026 Destroyer

use crate::dsk::{DirEntry, Dsk, DIR_ENTRY_SIZE, MAX_DIR_ENTRIES};
use crate::utils;
use anyhow::{Context, Result, bail};
use colored::*;
use std::path::PathBuf;

pub struct AttrOptions {
    pub read_only: Option<bool>,  // Some(true) = set, Some(false) = clear, None = unchanged
    pub system: Option<bool>,
}

pub fn execute(image: &PathBuf, file: &str, opts: AttrOptions) -> Result<()> {
    if opts.read_only.is_none() && opts.system.is_none() {
        bail!("No attribute specified. Use --read-only, --no-read-only, --system or --no-system.");
    }

    let mut dsk =
        Dsk::open(image).context(format!("Failed to open DSK image: {}", image.display()))?;

    // Read directory blocks
    let mut dir_data = Vec::with_capacity(MAX_DIR_ENTRIES * DIR_ENTRY_SIZE);
    dir_data.extend_from_slice(&dsk.read_block(0)?);
    dir_data.extend_from_slice(&dsk.read_block(1)?);

    let target = file.to_uppercase();
    let mut found = false;

    for i in 0..MAX_DIR_ENTRIES {
        let offset = i * DIR_ENTRY_SIZE;
        if offset + DIR_ENTRY_SIZE > dir_data.len() {
            break;
        }

        let mut entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE])?;
        if entry.is_deleted() {
            continue;
        }

        let name = utils::from_amsdos_name(&[&entry.name[..], &entry.ext[..]].concat());
        if name.to_uppercase() != target {
            continue;
        }

        // Modify attribute bits (bit 7 of first/second extension byte)
        if let Some(ro) = opts.read_only {
            if ro {
                entry.ext[0] |= 0x80;
            } else {
                entry.ext[0] &= 0x7F;
            }
        }
        if let Some(sys) = opts.system {
            if sys {
                entry.ext[1] |= 0x80;
            } else {
                entry.ext[1] &= 0x7F;
            }
        }

        dsk.write_dir_entry(i, &entry)?;
        found = true;
    }

    if !found {
        bail!("File not found: {}", file);
    }

    dsk.save(image).context("Failed to save DSK image")?;

    println!("{} Attributes updated for {}", "✓".bright_green().bold(), file.bright_white());
    Ok(())
}
