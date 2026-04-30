// MIT License - Copyright (c) 2026 Destroyer

use crate::amsdos::{has_header, AmsdosHeader};
use crate::cli::{FileType, ImportOptions};
use crate::dsk::{DirEntry, Dsk, BLOCK_SIZE};
use crate::utils;
use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::PathBuf;

pub fn execute(image: &PathBuf, files: Vec<PathBuf>, options: ImportOptions) -> Result<()> {
    let mut dsk =
        Dsk::open(image).context(format!("Failed to open DSK image: {}", image.display()))?;

    println!("{} Importing files to DSK...", "→".bright_cyan().bold());
    println!();

    let mut imported_count = 0;
    let mut total_size = 0;

    for file_path in &files {
        if !file_path.exists() {
            eprintln!(
                "{} File not found: {}",
                "✗".bright_red(),
                file_path.display()
            );
            continue;
        }

        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");

        match import_file(&mut dsk, file_path, filename, &options) {
            Ok(size) => {
                println!(
                    "  {} {} ({} bytes)",
                    "✓".bright_green(),
                    filename.bright_white(),
                    size
                );
                imported_count += 1;
                total_size += size;
            }
            Err(e) => {
                eprintln!("  {} {} - {}", "✗".bright_red(), filename, e);
            }
        }
    }

    if imported_count > 0 {
        dsk.save(image).context("Failed to save DSK image")?;

        println!();
        println!(
            "{} Imported {} file(s), {} total",
            "✓".bright_green().bold(),
            imported_count,
            utils::format_size(total_size)
        );
    } else {
        println!();
        println!("{} No files imported", "○".bright_yellow());
    }

    Ok(())
}

fn import_file(
    dsk: &mut Dsk,
    file_path: &PathBuf,
    filename: &str,
    options: &ImportOptions,
) -> Result<usize> {
    // Read file data
    let mut file_data =
        fs::read(file_path).context(format!("Failed to read file: {}", file_path.display()))?;

    // Determine file type
    let file_type = options.file_type.clone().unwrap_or_else(|| {
        if has_header(&file_data) {
            FileType::Binary
        } else {
            FileType::Ascii
        }
    });

    // Handle AMSDOS header
    let has_existing_header = has_header(&file_data);

    match file_type {
        FileType::Ascii => {
            // Remove header if present
            if has_existing_header {
                file_data = file_data[128..].to_vec();
            }
            // Convert LF to CRLF (DOS format) - required by Amstrad CPC
            file_data = lf_to_crlf(&file_data);
        }
        FileType::Binary => {
            // Add header if not present
            if !has_existing_header {
                let mut header = AmsdosHeader::new(filename, crate::amsdos::FileType::Binary);
                header.logical_length = file_data.len() as u16;
                header.real_length = file_data.len() as u16;

                if let Some(load) = options.load {
                    header.load_address = load;
                }
                if let Some(exec) = options.exec {
                    header.entry_address = exec;
                }

                let header_bytes = header.to_bytes();
                let mut new_data = Vec::with_capacity(header_bytes.len() + file_data.len());
                new_data.extend_from_slice(&header_bytes);
                new_data.extend_from_slice(&file_data);
                file_data = new_data;
            }
        }
        FileType::Raw => {
            // Keep as-is
        }
    }

    let file_size = file_data.len();

    // Check if file already exists
    let amsdos_name = utils::to_amsdos_name(filename);
    let catalog = dsk.catalog()?;

    for entry in &catalog.entries {
        if utils::to_amsdos_name(&entry.name) == amsdos_name {
            if !options.force {
                anyhow::bail!("File already exists (use --force to overwrite)");
            }
            // TODO: Remove existing file first
            break;
        }
    }

    // Get block bitmap
    let mut bitmap = dsk.get_block_bitmap()?;

    // real_size = tamaño original antes del padding
    let real_size = file_size;

    // Calculate number of blocks needed (1 block = 1024 bytes)
    let num_blocks = real_size.div_ceil(BLOCK_SIZE);

    // Check if we have enough space
    let free_blocks: usize = bitmap.iter().filter(|&&used| !used).count();
    if free_blocks < num_blocks {
        anyhow::bail!(
            "Not enough space on disk (need {} blocks, have {})",
            num_blocks,
            free_blocks
        );
    }

    // Pad file data to block boundary with 0x00 for ASCII, 0xE5 for binary
    let pad_byte = match options.file_type.clone().unwrap_or(FileType::Ascii) {
        FileType::Binary => 0xE5u8,
        _ => 0x00u8,
    };
    let padded_size = num_blocks * BLOCK_SIZE;
    file_data.resize(padded_size, pad_byte);

    // Write file in extents (max 16 blocks per extent = 16KB)
    let mut file_pos = 0;
    let mut extent_num = 0;
    // remaining_pages = número real de páginas de 128 bytes del archivo original
    let total_pages = real_size.div_ceil(128);
    let mut remaining_pages = total_pages;

    while file_pos < file_data.len() {
        let dir_index = dsk.find_free_dir_entry()?;

        // Blocks in this extent (max 16)
        let blocks_in_extent = std::cmp::min(16, (file_data.len() - file_pos) / BLOCK_SIZE);

        // Pages in this extent = real pages remaining, max 128 (16 blocks * 8 pages)
        let pages_in_extent = std::cmp::min(remaining_pages, blocks_in_extent * 8);
        remaining_pages = remaining_pages.saturating_sub(pages_in_extent);

        // Find free blocks
        let mut extent_blocks = Vec::new();
        for (i, used) in bitmap.iter_mut().enumerate().skip(2) {
            if !*used && extent_blocks.len() < blocks_in_extent {
                extent_blocks.push(i as u8);
                *used = true;
            }
        }

        if extent_blocks.len() < blocks_in_extent {
            anyhow::bail!("Failed to allocate blocks");
        }

        // Write blocks
        for (i, &block_num) in extent_blocks.iter().enumerate() {
            let block_start = file_pos + (i * BLOCK_SIZE);
            let block_end = std::cmp::min(block_start + BLOCK_SIZE, file_data.len());
            let mut block_data = vec![pad_byte; BLOCK_SIZE];
            block_data[0..(block_end - block_start)]
                .copy_from_slice(&file_data[block_start..block_end]);
            dsk.write_block(block_num, &block_data)?;
        }

        // Create directory entry
        let mut dir_entry = DirEntry::new();
        dir_entry.user = options.user;

        // Set filename
        let name_parts: Vec<&str> = filename.split('.').collect();
        let basename = name_parts[0].to_uppercase();
        let ext = if name_parts.len() > 1 {
            name_parts[1].to_uppercase()
        } else {
            String::new()
        };

        for (i, c) in basename.chars().take(8).enumerate() {
            dir_entry.name[i] = c as u8;
        }
        for (i, c) in ext.chars().take(3).enumerate() {
            dir_entry.ext[i] = c as u8;
        }

        // Set attributes
        if options.read_only {
            dir_entry.ext[0] |= 0x80;
        }
        if options.system {
            dir_entry.ext[1] |= 0x80;
        }

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

    Ok(file_size)
}

/// Convert LF to CRLF (DOS format) - required by Amstrad CPC for ASCII files
fn lf_to_crlf(data: &[u8]) -> Vec<u8> {
    let mut result = Vec::with_capacity(data.len() + 32);
    for &byte in data {
        if byte == b'\n' && result.last() != Some(&b'\r') {
            result.push(b'\r');
        }
        result.push(byte);
    }
    result
}
