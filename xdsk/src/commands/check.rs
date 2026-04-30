// MIT License - Copyright (c) 2026 Destroyer

use crate::dsk::{DirEntry, Dsk, DIR_ENTRY_SIZE, MAX_DIR_ENTRIES};
use anyhow::{anyhow, Result};
use colored::*;
use std::collections::HashSet;
use std::path::PathBuf;

/// Execute `disc check <image>`.
///
/// Validates the header, directory entries and block allocation of a DSK image.
/// Returns an error (causing exit code 1) if any issue is found.
pub fn execute(image: &PathBuf) -> Result<()> {
    println!("Checking {}...\n", image.display());

    let mut issues: usize = 0;

    // ── Open & validate header ────────────────────────────────────────────────
    let dsk = match Dsk::open(image) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{} Failed to open: {}", "✗".bright_red(), e);
            return Err(anyhow!("Check failed: 1 error(s) found"));
        }
    };

    println!("Header");

    let header = dsk.header();
    let format_name = if header.magic.starts_with("MV -") {
        "DATA format"
    } else if header.magic.starts_with("EXTENDED CPC DSK") {
        "EXTENDED format"
    } else {
        "unknown format"
    };

    match dsk.validate() {
        Ok(_) => {
            println!("  {} Magic valid ({})", "✓".bright_green(), format_name);
            println!(
                "  {} {} tracks, {} head(s)",
                "✓".bright_green(),
                header.tracks,
                header.heads
            );
        }
        Err(e) => {
            println!("  {} Header invalid: {}", "✗".bright_red(), e);
            issues += 1;
        }
    }

    log::debug!("Header check done — {} issue(s) so far", issues);

    // ── Directory ─────────────────────────────────────────────────────────────
    println!("\nDirectory");

    let dir_data_result = dsk
        .read_block(0)
        .and_then(|b0| dsk.read_block(1).map(|b1| [b0, b1].concat()));

    match dir_data_result {
        Err(e) => {
            println!("  {} Cannot read directory: {}", "✗".bright_red(), e);
            issues += 1;
        }
        Ok(dir_data) => {
            let mut entries_used: usize = 0;
            let mut used_blocks: HashSet<u8> = HashSet::new();
            let mut block_conflicts: usize = 0;

            for i in 0..MAX_DIR_ENTRIES {
                let offset = i * DIR_ENTRY_SIZE;
                if offset + DIR_ENTRY_SIZE > dir_data.len() {
                    break;
                }

                let entry = match DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE]) {
                    Ok(e) => e,
                    Err(e) => {
                        println!("  {} Entry {}: parse error: {}", "✗".bright_red(), i, e);
                        issues += 1;
                        continue;
                    }
                };

                if !entry.is_deleted() {
                    entries_used += 1;
                    for &block in &entry.blocks {
                        if block > 1 && !used_blocks.insert(block) {
                            block_conflicts += 1;
                        }
                    }
                }
            }

            println!(
                "  {} {} / {} entries used",
                "✓".bright_green(),
                entries_used,
                MAX_DIR_ENTRIES
            );

            // Per-file header report
            let catalog = dsk.catalog()?;
            for cat_entry in &catalog.entries {
                let header_result = file_has_amsdos_header(&dsk, &dir_data, &cat_entry.name);
                let (icon, header_info) = match header_result {
                    Some(true)  => ("✓".bright_green(),  "AMSDOS header valid"),
                    Some(false) => ("✓".bright_green(),  "no AMSDOS header (ASCII/raw)"),
                    None        => ("✗".bright_red(),    "cannot read first block"),
                };
                if header_result.is_none() {
                    issues += 1;
                }
                println!(
                    "  {} {} \u{2014} {}",
                    icon,
                    cat_entry.name.bright_white(),
                    header_info
                );
                log::debug!("  {} — {}", cat_entry.name, header_info);
            }

            if block_conflicts > 0 {
                println!(
                    "  {} Block allocation: {} conflict(s) detected",
                    "✗".bright_red(),
                    block_conflicts
                );
                issues += block_conflicts;
            } else {
                println!(
                    "  {} Block allocation OK (no conflicts)",
                    "✓".bright_green()
                );
            }
        }
    }

    // ── Result ────────────────────────────────────────────────────────────────
    println!();
    if issues == 0 {
        println!("{} OK \u{2014} 0 issues", "Result:".bright_white().bold());
        Ok(())
    } else {
        println!(
            "{} {} issue(s) found",
            "Result:".bright_white().bold(),
            issues
        );
        Err(anyhow!("Check failed: {} error(s) found", issues))
    }
}

/// Returns `Some(true)` if the file's first block contains a valid AMSDOS
/// header, `Some(false)` if not, or `None` if the block cannot be read.
fn file_has_amsdos_header(dsk: &Dsk, dir_data: &[u8], filename: &str) -> Option<bool> {
    for i in 0..MAX_DIR_ENTRIES {
        let offset = i * DIR_ENTRY_SIZE;
        if offset + DIR_ENTRY_SIZE > dir_data.len() {
            break;
        }

        let entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE]).ok()?;

        if entry.is_deleted() || !entry.is_first_extent() {
            continue;
        }

        let entry_name =
            crate::utils::from_amsdos_name(&[&entry.name[..], &entry.ext[..]].concat());
        if entry_name != filename || entry.blocks[0] == 0 {
            continue;
        }

        return dsk
            .read_block(entry.blocks[0])
            .ok()
            .map(|block| crate::amsdos::has_header(&block));
    }
    None
}
