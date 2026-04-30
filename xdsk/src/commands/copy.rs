// MIT License - Copyright (c) 2026 Destroyer

use crate::dsk::{DirEntry, Dsk, BLOCK_SIZE, DIR_ENTRY_SIZE, MAX_DIR_ENTRIES, USER_DELETED};
use crate::utils;
use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;

/// Copy files from one DSK image to another, preserving AMSDOS headers and attributes.
pub fn execute(src: &PathBuf, dst: &PathBuf, files: Vec<String>, force: bool) -> Result<()> {
    let src_dsk =
        Dsk::open(src).with_context(|| format!("Failed to open source DSK: {}", src.display()))?;
    let mut dst_dsk = Dsk::open(dst)
        .with_context(|| format!("Failed to open destination DSK: {}", dst.display()))?;

    // Empty file list → copy everything
    let patterns: Vec<String> = if files.is_empty() {
        vec!["*".to_string()]
    } else {
        files
    };

    println!(
        "{} Copying {} → {}...",
        "→".bright_cyan().bold(),
        src.display(),
        dst.display()
    );
    println!();

    let mut copied = 0usize;
    let mut total_size = 0usize;

    for pattern in &patterns {
        let matches = find_matching_files(&src_dsk, pattern)?;

        if matches.is_empty() {
            if pattern != "*" {
                eprintln!("{} No files match pattern: {}", "✗".bright_red(), pattern);
            }
            continue;
        }

        for filename in matches {
            match copy_file(&src_dsk, &mut dst_dsk, &filename, force) {
                Ok(size) => {
                    println!(
                        "  {} {} ({})",
                        "✓".bright_green(),
                        filename.bright_white(),
                        utils::format_size(size)
                    );
                    copied += 1;
                    total_size += size;
                }
                Err(e) => {
                    eprintln!("  {} {} — {}", "✗".bright_red(), filename, e);
                }
            }
        }
    }

    if copied > 0 {
        dst_dsk
            .save(dst)
            .context("Failed to save destination DSK")?;
        println!();
        println!(
            "{} Copied {} file(s), {} total",
            "✓".bright_green().bold(),
            copied,
            utils::format_size(total_size)
        );
    } else {
        println!();
        println!("{} No files copied", "○".bright_yellow());
    }

    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Internal helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Return a deduplicated list of file names in `dsk` that match `pattern`.
///
/// Supported wildcard forms:
/// - `*`       — match everything
/// - `*.EXT`   — match any file with that extension (e.g. `*.BAS`)
/// - `NAME*`   — match any file whose name starts with `NAME` (e.g. `LEVEL*`)
/// - `EXACT`   — exact name match (case-insensitive)
///
/// Only first-extent directory entries are considered.
fn find_matching_files(dsk: &Dsk, pattern: &str) -> Result<Vec<String>> {
    let pattern_upper = pattern.to_uppercase();

    // Determine match mode from the pattern shape
    enum Mode {
        All,              // "*"
        ByExt(String),    // "*.EXT"  → match extension
        ByPrefix(String), // "NAME*"  → match name prefix
        Exact(String),    // "EXACT"  → exact match
    }

    let mode = if pattern_upper == "*" {
        Mode::All
    } else if let Some(ext) = pattern_upper.strip_prefix("*.") {
        Mode::ByExt(ext.to_string())
    } else if pattern_upper.ends_with('*') {
        Mode::ByPrefix(pattern_upper.trim_end_matches('*').to_string())
    } else {
        Mode::Exact(pattern_upper)
    };

    let mut dir_data = Vec::with_capacity(MAX_DIR_ENTRIES * DIR_ENTRY_SIZE);
    dir_data.extend_from_slice(&dsk.read_block(0)?);
    dir_data.extend_from_slice(&dsk.read_block(1)?);

    let mut seen = std::collections::HashSet::new();
    let mut matches = Vec::new();

    for i in 0..MAX_DIR_ENTRIES {
        let offset = i * DIR_ENTRY_SIZE;
        if offset + DIR_ENTRY_SIZE > dir_data.len() {
            break;
        }
        let entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE])?;
        if entry.is_deleted() || !entry.is_first_extent() {
            continue;
        }

        let name = utils::from_amsdos_name(&[&entry.name[..], &entry.ext[..]].concat());
        let name_upper = name.to_uppercase();

        let hit = match &mode {
            Mode::All => true,
            Mode::ByExt(ext) => {
                // "LEVEL1.BAS" → extension is "BAS"
                name_upper
                    .rsplit_once('.')
                    .map(|(_, e)| e == ext.as_str())
                    .unwrap_or(false)
            }
            Mode::ByPrefix(prefix) => name_upper.starts_with(prefix.as_str()),
            Mode::Exact(exact) => name_upper == exact.as_str(),
        };

        if hit && !seen.contains(&name_upper) {
            seen.insert(name_upper);
            matches.push(name);
        }
    }

    Ok(matches)
}

/// Find the first directory extent of `filename`, masking attribute bits in the comparison.
fn find_first_entry(dsk: &Dsk, filename: &str) -> Result<Option<DirEntry>> {
    let parts: Vec<&str> = filename.split('.').collect();

    let mut expected_name = [b' '; 8];
    for (i, c) in parts[0].chars().take(8).enumerate() {
        expected_name[i] = c.to_ascii_uppercase() as u8;
    }

    let mut expected_ext = [b' '; 3];
    if parts.len() > 1 {
        for (i, c) in parts[1].chars().take(3).enumerate() {
            expected_ext[i] = c.to_ascii_uppercase() as u8;
        }
    }

    let mut dir_data = Vec::with_capacity(MAX_DIR_ENTRIES * DIR_ENTRY_SIZE);
    dir_data.extend_from_slice(&dsk.read_block(0)?);
    dir_data.extend_from_slice(&dsk.read_block(1)?);

    for i in 0..MAX_DIR_ENTRIES {
        let offset = i * DIR_ENTRY_SIZE;
        if offset + DIR_ENTRY_SIZE > dir_data.len() {
            break;
        }
        let entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE])?;
        if entry.is_deleted() || !entry.is_first_extent() {
            continue;
        }

        // Mask out bit 7 (attribute flags) before comparing
        let name_ok = entry
            .name
            .iter()
            .zip(expected_name.iter())
            .all(|(a, b)| a & 0x7F == *b);
        let ext_ok = entry
            .ext
            .iter()
            .zip(expected_ext.iter())
            .all(|(a, b)| a & 0x7F == *b);

        if name_ok && ext_ok {
            return Ok(Some(entry));
        }
    }

    Ok(None)
}

/// Mark all directory extents of `filename` as deleted in `dst_dsk`.
fn remove_from_dst(dst_dsk: &mut Dsk, filename: &str) -> Result<()> {
    let parts: Vec<&str> = filename.split('.').collect();

    let mut expected_name = [b' '; 8];
    for (i, c) in parts[0].chars().take(8).enumerate() {
        expected_name[i] = c.to_ascii_uppercase() as u8;
    }

    let mut expected_ext = [b' '; 3];
    if parts.len() > 1 {
        for (i, c) in parts[1].chars().take(3).enumerate() {
            expected_ext[i] = c.to_ascii_uppercase() as u8;
        }
    }

    let mut dir_data = Vec::with_capacity(MAX_DIR_ENTRIES * DIR_ENTRY_SIZE);
    dir_data.extend_from_slice(&dst_dsk.read_block(0)?);
    dir_data.extend_from_slice(&dst_dsk.read_block(1)?);

    for i in 0..MAX_DIR_ENTRIES {
        let offset = i * DIR_ENTRY_SIZE;
        if offset + DIR_ENTRY_SIZE > dir_data.len() {
            break;
        }
        let entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE])?;
        if entry.is_deleted() {
            continue;
        }

        let name_ok = entry
            .name
            .iter()
            .zip(expected_name.iter())
            .all(|(a, b)| a & 0x7F == *b);
        let ext_ok = entry
            .ext
            .iter()
            .zip(expected_ext.iter())
            .all(|(a, b)| a & 0x7F == *b);

        if name_ok && ext_ok {
            let mut deleted = DirEntry::new();
            deleted.user = USER_DELETED;
            dst_dsk.write_dir_entry(i, &deleted)?;
        }
    }

    Ok(())
}

/// Copy a single file from `src_dsk` to `dst_dsk`.
fn copy_file(src_dsk: &Dsk, dst_dsk: &mut Dsk, filename: &str, force: bool) -> Result<usize> {
    log::debug!("copy_file: '{}'", filename);

    // Read complete file bytes from source (AMSDOS header included if present)
    let file_data = src_dsk
        .read_file_data(filename)
        .with_context(|| format!("Failed to read '{}' from source", filename))?
        .ok_or_else(|| anyhow::anyhow!("'{}' not found in source DSK", filename))?;

    // Retrieve source directory entry to preserve user / attribute bits
    let src_entry = find_first_entry(src_dsk, filename)?
        .ok_or_else(|| anyhow::anyhow!("Directory entry for '{}' not found", filename))?;

    // Check whether the file already exists in the destination
    let dst_catalog = dst_dsk.catalog()?;
    let amsdos_name = utils::to_amsdos_name(filename);
    let exists = dst_catalog
        .entries
        .iter()
        .any(|e| utils::to_amsdos_name(&e.name) == amsdos_name);

    if exists {
        if !force {
            anyhow::bail!(
                "'{}' already exists in destination (use --force to overwrite)",
                filename
            );
        }
        log::debug!(
            "copy_file: removing existing '{}' from destination",
            filename
        );
        remove_from_dst(dst_dsk, filename)?;
    }

    let size = file_data.len();
    write_file_to_dsk(dst_dsk, filename, &file_data, &src_entry)?;
    Ok(size)
}

/// Write raw file bytes into `dsk`, reconstructing directory entries that mirror
/// the attributes stored in `src_entry`.
fn write_file_to_dsk(
    dsk: &mut Dsk,
    filename: &str,
    file_data: &[u8],
    src_entry: &DirEntry,
) -> Result<()> {
    let real_size = file_data.len();
    let num_blocks = real_size.div_ceil(BLOCK_SIZE);

    let mut bitmap = dsk.get_block_bitmap()?;
    let free_blocks = bitmap.iter().filter(|&&u| !u).count();
    if free_blocks < num_blocks {
        anyhow::bail!(
            "Not enough space in destination (need {} blocks, have {})",
            num_blocks,
            free_blocks
        );
    }

    // Pad to a full block boundary
    let padded_size = num_blocks * BLOCK_SIZE;
    let mut padded = file_data.to_vec();
    padded.resize(padded_size, 0xE5u8);

    // Split "NAME.EXT" for directory entry fields
    let parts: Vec<&str> = filename.split('.').collect();
    let basename = parts[0].to_uppercase();
    let ext_str = if parts.len() > 1 {
        parts[1].to_uppercase()
    } else {
        String::new()
    };

    // Write in extents (max 16 blocks = 16 KB each)
    let mut file_pos = 0usize;
    let mut extent_num = 0u8;
    let total_pages = real_size.div_ceil(128);
    let mut remaining_pages = total_pages;

    while file_pos < padded.len() {
        let dir_index = dsk.find_free_dir_entry()?;

        let blocks_in_extent = std::cmp::min(16, (padded.len() - file_pos) / BLOCK_SIZE);
        let pages_in_extent = std::cmp::min(remaining_pages, blocks_in_extent * 8);
        remaining_pages = remaining_pages.saturating_sub(pages_in_extent);

        // Allocate free blocks (skip 0 and 1 — reserved for the directory)
        let mut extent_blocks: Vec<u8> = Vec::new();
        for (i, used) in bitmap.iter_mut().enumerate().skip(2) {
            if !*used && extent_blocks.len() < blocks_in_extent {
                extent_blocks.push(i as u8);
                *used = true;
            }
        }

        if extent_blocks.len() < blocks_in_extent {
            anyhow::bail!("Failed to allocate blocks in destination");
        }

        // Write data blocks
        for (i, &block_num) in extent_blocks.iter().enumerate() {
            let start = file_pos + i * BLOCK_SIZE;
            let end = std::cmp::min(start + BLOCK_SIZE, padded.len());
            let mut block_data = vec![0xE5u8; BLOCK_SIZE];
            block_data[..end - start].copy_from_slice(&padded[start..end]);
            dsk.write_block(block_num, &block_data)?;
        }

        // Build directory entry
        let mut dir_entry = DirEntry::new();
        dir_entry.user = src_entry.user;

        for (i, c) in basename.chars().take(8).enumerate() {
            dir_entry.name[i] = c as u8;
        }
        for (i, c) in ext_str.chars().take(3).enumerate() {
            dir_entry.ext[i] = c as u8;
        }
        // Restore attribute bits (read-only bit 7 of ext[0], system bit 7 of ext[1])
        dir_entry.ext[0] |= src_entry.ext[0] & 0x80;
        dir_entry.ext[1] |= src_entry.ext[1] & 0x80;
        dir_entry.ext[2] |= src_entry.ext[2] & 0x80;

        dir_entry.page_num = extent_num;
        dir_entry.num_pages = pages_in_extent as u8;

        for (i, &block) in extent_blocks.iter().enumerate() {
            if i < 16 {
                dir_entry.blocks[i] = block;
            }
        }

        dsk.write_dir_entry(dir_index, &dir_entry)?;

        file_pos += blocks_in_extent * BLOCK_SIZE;
        extent_num += 1;
    }

    Ok(())
}
