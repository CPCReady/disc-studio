// MIT License — Copyright (c) Destroyer 2026.
//
// `xcart info` — display metadata for a DSK or CPR file.
// File type is auto-detected from the extension (falling back to magic bytes).

use std::path::Path;

use crate::cpr::read_cpr;
use crate::dsk::DskFile;
use crate::error::Result;

pub fn run(input: &Path) -> Result<()> {
    let ext = input
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "dsk" => info_dsk(input),
        "cpr" => info_cpr(input),
        _ => {
            // Fallback: sniff magic bytes
            let bytes = std::fs::read(input)?;
            if bytes.starts_with(b"RIFF") {
                info_cpr(input)
            } else {
                info_dsk(input)
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────

fn info_dsk(path: &Path) -> Result<()> {
    let dsk = DskFile::open(path)?;
    let meta = std::fs::metadata(path)?;

    println!("─── DSK Disk Image ──────────────────────────────────");
    println!("  File       : {}", path.display());
    println!("  Format     : {}", dsk.format);
    println!(
        "  Creator    : {}",
        if dsk.creator.is_empty() {
            "<unknown>"
        } else {
            &dsk.creator
        }
    );
    println!("  Tracks     : {}", dsk.tracks_count);
    println!("  Sides      : {}", dsk.sides_count);
    println!("  Tracks present: {}", dsk.tracks.len());
    println!("  Sectors    : {}", dsk.total_sectors());
    println!("  File size  : {} bytes", meta.len());

    if let Some(first) = dsk.tracks.first() {
        println!("  Sector size: {} bytes", first.sector_size);
        println!("  Sectors/track: {}", first.sectors.len());
        println!("  Min sector id: 0x{:02X}", dsk.min_sector_id());
    }

    Ok(())
}

fn info_cpr(path: &Path) -> Result<()> {
    let chunks = read_cpr(path)?;
    let meta = std::fs::metadata(path)?;

    println!("─── CPR Cartridge (RIFF/AMS!) ───────────────────────");
    println!("  File    : {}", path.display());
    println!("  Chunks  : {}", chunks.len());
    println!("  Size    : {} bytes", meta.len());

    if !chunks.is_empty() {
        println!();
        println!(
            "  {:<5}  {:<6}  {:>12}  Description",
            "Idx", "Tag", "Size"
        );
        println!("  {}", "─".repeat(42));
        for chunk in &chunks {
            println!(
                "  {:<5}  {:<6}  0x{:08X}  {}",
                chunk.index,
                chunk.tag_str(),
                chunk.size,
                chunk_description(chunk.index)
            );
        }
    }

    Ok(())
}

fn chunk_description(index: usize) -> &'static str {
    match index {
        0 => "OS ROM",
        1 => "BASIC ROM",
        2 => "AMSDOS ROM",
        _ => "Data chunk",
    }
}
