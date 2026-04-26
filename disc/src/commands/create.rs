use crate::cli::CreateOptions;
use crate::dsk::Dsk;
use anyhow::Result;
use colored::*;
use std::path::PathBuf;

pub fn execute(image: &PathBuf, options: CreateOptions) -> Result<()> {
    // Check if file exists
    if image.exists() && !options.force {
        anyhow::bail!(
            "File already exists: {}. Use --force to overwrite.",
            image.display()
        );
    }

    println!("{} Creating new DSK image...", "→".bright_cyan().bold());
    println!("  Tracks: {}", options.tracks);
    println!("  Sectors: {}", options.sectors);

    let dsk = Dsk::create(options.tracks, options.sectors)?;
    dsk.save(image)?;

    println!(
        "{} DSK image created: {}",
        "✓".bright_green().bold(),
        image.display()
    );

    Ok(())
}
