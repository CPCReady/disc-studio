// MIT License - Copyright (c) 2026 Destroyer

use crate::dsk::{Dsk, MAX_DIR_ENTRIES};
use anyhow::Result;
use colored::*;
use std::path::PathBuf;

/// Execute `disc info <image>`.
///
/// Prints detailed statistics about a DSK image: format, geometry, directory
/// occupancy, block-map usage and a file listing with types and attributes.
pub fn execute(image: &PathBuf) -> Result<()> {
    let dsk = Dsk::open(image)?;
    let header = dsk.header();
    let catalog = dsk.catalog()?;
    let bitmap = dsk.get_block_bitmap()?;

    let sectors_pt = dsk.sectors_per_track();
    let total_blocks = dsk.total_blocks();
    // Count only blocks within the valid range (bitmap has 256 entries)
    let used_blocks = bitmap[..total_blocks.min(bitmap.len())]
        .iter()
        .filter(|&&b| b)
        .count();
    let free_blocks = total_blocks.saturating_sub(used_blocks);

    let format_name = if header.magic.starts_with("MV -") {
        "DATA (standard CPC)"
    } else {
        "EXTENDED CPC DSK"
    };

    log::debug!(
        "info: {} tracks, {} sect/trk, {} total blocks, {} used, {} free",
        header.tracks,
        sectors_pt,
        total_blocks,
        used_blocks,
        free_blocks
    );

    // ── Header ────────────────────────────────────────────────────────────────
    println!("DSK Image: {}", image.display());
    println!("{}", "\u{2501}".repeat(40));
    println!("  {:<16} {}", "Format", format_name);
    println!("  {:<16} {}", "Tracks", header.tracks);
    println!("  {:<16} {}", "Sectors/Track", sectors_pt);
    println!("  {:<16} 512 bytes", "Sector size");
    println!("  {:<16} {} KB", "Capacity", total_blocks);

    // ── Directory ─────────────────────────────────────────────────────────────
    println!("Directory");
    println!(
        "  {:<16} {} / {}",
        "Entries used",
        catalog.entries.len(),
        MAX_DIR_ENTRIES
    );

    // ── Block Map ─────────────────────────────────────────────────────────────
    let used_pct = if total_blocks > 0 {
        (used_blocks as f64 / total_blocks as f64) * 100.0
    } else {
        0.0
    };
    let free_pct = 100.0_f64 - used_pct;

    println!("Block Map");
    println!("  {:<16} {}", "Total blocks", total_blocks);
    println!("  {:<16} {}  ({:.1}%)", "Used", used_blocks, used_pct);
    println!("  {:<16} {} ({:.1}%)", "Free", free_blocks, free_pct);

    // ── File listing ──────────────────────────────────────────────────────────
    if !catalog.entries.is_empty() {
        println!("Files");
        println!("  {:<14} {:<10} {:<10} Attr", "Name", "Type", "Size");
        println!("  {}", "\u{2500}".repeat(46));

        for entry in &catalog.entries {
            let attrs = {
                let mut a = String::new();
                if entry.read_only {
                    a.push('R');
                }
                if entry.system {
                    a.push('S');
                }
                if a.is_empty() {
                    a.push('-');
                }
                a
            };
            let size_str = format_size(entry.size);
            println!(
                "  {:<14} {:<10} {:<10} {}",
                entry.name.bright_white(),
                entry.file_type,
                size_str,
                attrs
            );
        }
    }
    Ok(())
}

fn format_size(bytes: usize) -> String {
    if bytes >= 1024 {
        format!("{} KB", bytes / 1024)
    } else {
        format!("{} B", bytes)
    }
}
