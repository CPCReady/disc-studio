// MIT License — Copyright (c) Destroyer 2026.
//
// `xcart list` — list chunks inside a CPR cartridge file.
// With -v / --verbose, shows a hex preview of the first 16 bytes of each chunk.

use std::path::Path;

use crate::cpr::read_cpr;
use crate::error::Result;

pub fn run(input: &Path, verbose: bool) -> Result<()> {
    let chunks = read_cpr(input)?;

    println!("CPR: {} — {} chunk(s)\n", input.display(), chunks.len());

    if chunks.is_empty() {
        println!("No chunks found.");
        return Ok(());
    }

    if verbose {
        println!(
            "{:<5}  {:<6}  {:>12}  {:<20}  First 16 bytes (hex)",
            "Idx", "Tag", "Size", "Description"
        );
        println!("{}", "─".repeat(76));
    } else {
        println!(
            "{:<5}  {:<6}  {:>12}  Description",
            "Idx", "Tag", "Size"
        );
        println!("{}", "─".repeat(46));
    }

    for chunk in &chunks {
        let desc = chunk_description(chunk.index);
        if verbose {
            let hex: String = chunk
                .data
                .iter()
                .take(16)
                .map(|b| format!("{:02X}", b))
                .collect::<Vec<_>>()
                .join(" ");
            println!(
                "{:<5}  {:<6}  0x{:08X}  {:<20}  {}",
                chunk.index,
                chunk.tag_str(),
                chunk.size,
                desc,
                hex
            );
        } else {
            println!(
                "{:<5}  {:<6}  0x{:08X}  {}",
                chunk.index,
                chunk.tag_str(),
                chunk.size,
                desc
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
