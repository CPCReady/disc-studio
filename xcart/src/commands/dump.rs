// MIT License — Copyright (c) Destroyer 2026.
//
// `xcart dump` — dump raw sector data from a DSK to a binary file.

use std::path::Path;

use crate::dsk::DskFile;
use crate::error::Result;

pub fn run(input: &Path, output: &Path) -> Result<()> {
    let dsk = DskFile::open(input)?;
    eprintln!("  Reading : {}", dsk);

    let data = dsk.collect_sector_data();
    std::fs::write(output, &data)?;

    eprintln!(
        "  Dumped  : {} sectors, {} bytes → {}",
        dsk.total_sectors(),
        data.len(),
        output.display()
    );

    Ok(())
}
