// MIT License - Copyright (c) 2026 Destroyer

use crate::dsk::{DirEntry, Dsk, DIR_ENTRY_SIZE, MAX_DIR_ENTRIES, USER_DELETED};
use crate::utils;
use anyhow::{Context, Result};
use colored::*;
use std::io::{self, Write};
use std::path::PathBuf;

pub fn execute(image: &PathBuf, files: Vec<String>, force: bool) -> Result<()> {
    let mut dsk =
        Dsk::open(image).context(format!("Failed to open DSK image: {}", image.display()))?;

    println!("{} Removing files from DSK...", "→".bright_cyan().bold());
    println!();

    let mut removed_count = 0;

    for file_pattern in &files {
        let matches = find_matching_files(&dsk, file_pattern)?;

        if matches.is_empty() {
            eprintln!("{} File not found: {}", "✗".bright_red(), file_pattern);
            continue;
        }

        for (name, entry_indices) in matches {
            if !force {
                print!("  Remove {} ? (y/N): ", name.bright_white());
                io::stdout().flush()?;

                let mut response = String::new();
                io::stdin().read_line(&mut response)?;

                if !response.trim().eq_ignore_ascii_case("y") {
                    println!("  {} Skipped", "○".bright_yellow());
                    continue;
                }
            }

            match remove_file(&mut dsk, &entry_indices) {
                Ok(()) => {
                    println!("  {} {}", "✓".bright_green(), name.bright_white());
                    removed_count += 1;
                }
                Err(e) => {
                    eprintln!("  {} {} - {}", "✗".bright_red(), name, e);
                }
            }
        }
    }

    if removed_count > 0 {
        dsk.save(image).context("Failed to save DSK image")?;

        println!();
        println!(
            "{} Removed {} file(s)",
            "✓".bright_green().bold(),
            removed_count
        );
    } else {
        println!();
        println!("{} No files removed", "○".bright_yellow());
    }

    Ok(())
}

fn find_matching_files(dsk: &Dsk, pattern: &str) -> Result<Vec<(String, Vec<usize>)>> {
    let mut matches: std::collections::HashMap<String, Vec<usize>> =
        std::collections::HashMap::new();

    // Simple wildcard matching (* at end)
    let is_wildcard = pattern.ends_with('*');
    let prefix = if is_wildcard {
        pattern.trim_end_matches('*').to_uppercase()
    } else {
        pattern.to_uppercase()
    };

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

        let name_upper = entry_name.to_uppercase();

        let matches_pattern = if is_wildcard {
            name_upper.starts_with(&prefix)
        } else {
            name_upper == prefix
        };

        if matches_pattern {
            matches.entry(entry_name).or_default().push(i);
        }
    }

    Ok(matches.into_iter().collect())
}

fn remove_file(dsk: &mut Dsk, entry_indices: &[usize]) -> Result<()> {
    // Mark all extents as deleted using write_dir_entry (handles sector interleaving)
    for &index in entry_indices {
        let mut deleted = DirEntry::new(); // user = USER_DELETED by default
        deleted.user = USER_DELETED;
        dsk.write_dir_entry(index, &deleted)
            .context(format!("Failed to delete directory entry {}", index))?;
    }
    Ok(())
}
