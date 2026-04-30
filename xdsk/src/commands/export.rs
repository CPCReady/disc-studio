// MIT License - Copyright (c) 2026 Destroyer

use crate::amsdos::{has_header, AmsdosHeader};
use crate::cli::ExportOptions;
use crate::dsk::{DirEntry, Dsk, DIR_ENTRY_SIZE, MAX_DIR_ENTRIES};
use crate::utils;
use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::{Path, PathBuf};

pub fn execute(
    image: &PathBuf,
    files: Vec<String>,
    output: Option<PathBuf>,
    options: ExportOptions,
) -> Result<()> {
    let dsk = Dsk::open(image).context(format!("Failed to open DSK image: {}", image.display()))?;

    let output_dir = output.unwrap_or_else(|| PathBuf::from("."));

    // Create output directory if it doesn't exist
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).context(format!(
            "Failed to create output directory: {}",
            output_dir.display()
        ))?;
    }

    println!("{} Exporting files from DSK...", "→".bright_cyan().bold());
    println!();

    let mut exported_count = 0;
    let mut total_size = 0;

    for file_pattern in &files {
        let matches = find_matching_files(&dsk, file_pattern)?;

        if matches.is_empty() {
            eprintln!("{} File not found: {}", "✗".bright_red(), file_pattern);
            continue;
        }

        for (name, entry_index) in matches {
            match export_file(&dsk, &name, entry_index, &output_dir, &options) {
                Ok(size) => {
                    println!(
                        "  {} {} ({} bytes)",
                        "✓".bright_green(),
                        name.bright_white(),
                        size
                    );
                    exported_count += 1;
                    total_size += size;
                }
                Err(e) => {
                    eprintln!("  {} {} - {}", "✗".bright_red(), name, e);
                }
            }
        }
    }

    println!();
    println!(
        "{} Exported {} file(s), {} total",
        "✓".bright_green().bold(),
        exported_count,
        utils::format_size(total_size)
    );

    Ok(())
}

fn find_matching_files(dsk: &Dsk, pattern: &str) -> Result<Vec<(String, usize)>> {
    let mut matches = Vec::new();
    let catalog = dsk.catalog()?;

    // Simple wildcard matching (* at end)
    let is_wildcard = pattern.ends_with('*');
    let prefix = if is_wildcard {
        pattern.trim_end_matches('*').to_uppercase()
    } else {
        pattern.to_uppercase()
    };

    for (i, entry) in catalog.entries.iter().enumerate() {
        let name_upper = entry.name.to_uppercase();

        let matches_pattern = if is_wildcard {
            name_upper.starts_with(&prefix)
        } else {
            name_upper == prefix
        };

        if matches_pattern {
            matches.push((entry.name.clone(), i));
        }
    }

    Ok(matches)
}

fn export_file(
    dsk: &Dsk,
    filename: &str,
    entry_index: usize,
    output_dir: &Path,
    options: &ExportOptions,
) -> Result<usize> {
    let mut file_data = Vec::new();
    let mut first_extent = true;
    let mut amsdos_header: Option<AmsdosHeader> = None;
    let mut total_pages: usize = 0; // Accumulate NbPages across all extents

    let catalog = dsk.catalog()?;
    let target_entry = &catalog.entries[entry_index];

    // Read directory via block reads to correctly handle sector interleaving
    let mut dir_data = Vec::with_capacity(MAX_DIR_ENTRIES * DIR_ENTRY_SIZE);
    dir_data.extend_from_slice(&dsk.read_block(0)?);
    dir_data.extend_from_slice(&dsk.read_block(1)?);

    for i in 0..MAX_DIR_ENTRIES {
        let offset = i * DIR_ENTRY_SIZE;
        if offset + DIR_ENTRY_SIZE > dir_data.len() {
            break;
        }

        let dir_entry = DirEntry::from_bytes(&dir_data[offset..offset + DIR_ENTRY_SIZE])?;

        if dir_entry.is_deleted() {
            continue;
        }

        let entry_name =
            utils::from_amsdos_name(&[&dir_entry.name[..], &dir_entry.ext[..]].concat());

        if entry_name != target_entry.name {
            continue;
        }

        // Accumulate real size from NbPages (each page = 128 bytes)
        total_pages += dir_entry.num_pages as usize;

        // Read blocks for this extent
        let num_blocks = dir_entry.num_pages.div_ceil(8);

        for j in 0..num_blocks as usize {
            if j < 16 && dir_entry.blocks[j] != 0 {
                let block_data = dsk.read_block(dir_entry.blocks[j])?;

                if first_extent && j == 0 {
                    if has_header(&block_data) {
                        amsdos_header = Some(AmsdosHeader::from_bytes(&block_data[..128])?);
                        if options.strip_header {
                            file_data.extend_from_slice(&block_data[128..]);
                        } else {
                            file_data.extend_from_slice(&block_data);
                        }
                    } else {
                        file_data.extend_from_slice(&block_data);
                    }
                    first_extent = false;
                } else {
                    file_data.extend_from_slice(&block_data);
                }
            }
        }
    }

    // Truncate to real size
    if let Some(ref header) = amsdos_header {
        // With AMSDOS header: use logical_length from header
        let actual_size = if options.strip_header {
            header.logical_length as usize
        } else {
            header.logical_length as usize + 128
        };
        if file_data.len() > actual_size {
            file_data.truncate(actual_size);
        }
    } else {
        // Without AMSDOS header: use NbPages * 128 as real size
        let actual_size = total_pages * 128;
        if file_data.len() > actual_size {
            file_data.truncate(actual_size);
        }
    }

    let output_path = output_dir.join(filename);
    fs::write(&output_path, &file_data)
        .context(format!("Failed to write file: {}", output_path.display()))?;

    Ok(file_data.len())
}
