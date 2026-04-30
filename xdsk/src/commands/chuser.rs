// MIT License - Copyright (c) 2026 Destroyer

use crate::dsk::{DirEntry, Dsk, DIR_ENTRY_SIZE, MAX_DIR_ENTRIES};
use crate::utils;
use anyhow::{bail, Context, Result};
use colored::*;
use std::path::PathBuf;

pub fn execute(image: &PathBuf, file: &str, new_user: u8) -> Result<()> {
    if new_user > 15 {
        bail!("User number must be between 0 and 15, got {}", new_user);
    }

    let mut dsk =
        Dsk::open(image).context(format!("Failed to open DSK image: {}", image.display()))?;

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

        entry.user = new_user;
        dsk.write_dir_entry(i, &entry)?;
        found = true;
    }

    if !found {
        bail!("File not found: {}", file);
    }

    dsk.save(image).context("Failed to save DSK image")?;

    println!(
        "{} User changed to {} for {}",
        "✓".bright_green().bold(),
        new_user,
        file.bright_white()
    );
    Ok(())
}
