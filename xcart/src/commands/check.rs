// MIT License — Copyright (c) Destroyer 2026.
//
// `xcart check` — validate the structure of a CPR cartridge file.

use std::path::Path;

use crate::cpr::{read_cpr, CHUNK_SIZE};
use crate::error::Result;

pub fn run(input: &Path) -> Result<()> {
    let chunks = read_cpr(input)?;

    if chunks.is_empty() {
        eprintln!("Warning: CPR file is structurally valid but contains no chunks.");
        return Ok(());
    }

    println!("File  : {}", input.display());
    println!(
        "Size  : {} bytes",
        std::fs::metadata(input).map(|m| m.len()).unwrap_or(0)
    );
    println!("Chunks: {}\n", chunks.len());

    println!("{:<5}  {:<6}  {:>12}  {:<10}  {}", "Idx", "Tag", "Size", "Status", "Description");
    println!("{}", "─".repeat(62));

    let mut all_ok = true;
    for chunk in &chunks {
        let (status, ok) = if chunk.size as usize == CHUNK_SIZE {
            ("OK", true)
        } else {
            all_ok = false;
            ("SIZE WARN", false)
        };
        let desc = chunk_description(chunk.index);
        println!(
            "{:<5}  {:<6}  0x{:08X}  {:<10}  {}{}",
            chunk.index,
            chunk.tag_str(),
            chunk.size,
            status,
            desc,
            if !ok {
                format!(" (expected 0x{:04X})", CHUNK_SIZE)
            } else {
                String::new()
            }
        );
    }

    println!();
    if all_ok {
        println!("File OK — {} chunks validated.", chunks.len());
    } else {
        println!("File has warnings — review size mismatches above.");
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
