// MIT License — Copyright (c) Destroyer 2026.
//
// `xcart extract` — extract a single chunk from a CPR cartridge to a file.

use std::path::Path;

use crate::cpr::read_cpr;
use crate::error::{Error, Result};

pub fn run(input: &Path, chunk_index: usize, output: &Path) -> Result<()> {
    let chunks = read_cpr(input)?;

    let chunk = chunks
        .iter()
        .find(|c| c.index == chunk_index)
        .ok_or_else(|| {
            Error::Cpr(format!(
                "Chunk {} not found — file has {} chunk(s)",
                chunk_index,
                chunks.len()
            ))
        })?;

    std::fs::write(output, &chunk.data)?;

    eprintln!(
        "  Extracted: chunk {} ({}) — {} bytes → {}",
        chunk_index,
        chunk.tag_str(),
        chunk.data.len(),
        output.display()
    );

    Ok(())
}
