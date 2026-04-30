// MIT License - Copyright (c) 2026 Destroyer

use crate::dsk::Dsk;
use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use std::path::PathBuf;

/// Execute `disc diff <image1> <image2>`.
///
/// Compares the file catalogs (and content) of two DSK images and reports
/// files that are unique to each image, modified, or identical.
pub fn execute(image1: &PathBuf, image2: &PathBuf) -> Result<()> {
    let dsk1 = Dsk::open(image1)?;
    let dsk2 = Dsk::open(image2)?;

    let cat1 = dsk1.catalog()?;
    let cat2 = dsk2.catalog()?;

    println!(
        "Comparing {} \u{2194} {}",
        image1.display(),
        image2.display()
    );
    println!();

    // Build name → size maps for quick lookup
    let map1: HashMap<String, usize> = cat1
        .entries
        .iter()
        .map(|e| (e.name.clone(), e.size))
        .collect();
    let map2: HashMap<String, usize> = cat2
        .entries
        .iter()
        .map(|e| (e.name.clone(), e.size))
        .collect();

    let mut only_in_1: Vec<(String, usize)> = Vec::new();
    let mut only_in_2: Vec<(String, usize)> = Vec::new();
    let mut modified: Vec<(String, usize, usize)> = Vec::new(); // (name, size1, size2)
    let mut identical: Vec<(String, usize)> = Vec::new();

    // Files only in DSK1, or in both
    for (name, &size1) in &map1 {
        if let Some(&size2) = map2.get(name) {
            // Present in both — compare content
            let data1 = dsk1.read_file_data(name)?.unwrap_or_default();
            let data2 = dsk2.read_file_data(name)?.unwrap_or_default();
            if data1 == data2 {
                identical.push((name.clone(), size1));
            } else {
                modified.push((name.clone(), size1, size2));
            }
        } else {
            only_in_1.push((name.clone(), size1));
        }
    }

    // Files only in DSK2
    for (name, &size2) in &map2 {
        if !map1.contains_key(name) {
            only_in_2.push((name.clone(), size2));
        }
    }

    // Sort for deterministic output
    only_in_1.sort_by(|a, b| a.0.cmp(&b.0));
    only_in_2.sort_by(|a, b| a.0.cmp(&b.0));
    modified.sort_by(|a, b| a.0.cmp(&b.0));
    identical.sort_by(|a, b| a.0.cmp(&b.0));

    // ── Report ────────────────────────────────────────────────────────────────
    if !only_in_1.is_empty() {
        println!("Only in {}:", image1.display());
        for (name, size) in &only_in_1 {
            println!("  {}  {}", name.bright_white(), format_size(*size));
        }
        println!();
    }

    if !only_in_2.is_empty() {
        println!("Only in {}:", image2.display());
        for (name, size) in &only_in_2 {
            println!("  {}  {}", name.bright_white(), format_size(*size));
        }
        println!();
    }

    if !modified.is_empty() {
        println!("Modified (same name, different content):");
        for (name, size1, size2) in &modified {
            println!(
                "  {}  {} \u{2192} {}",
                name.bright_white(),
                format_size(*size1),
                format_size(*size2)
            );
        }
        println!();
    }

    if !identical.is_empty() {
        println!("Identical:");
        for (name, size) in &identical {
            println!("  {}  {}", name.bright_white(), format_size(*size));
        }
        println!();
    }

    // ── Summary ───────────────────────────────────────────────────────────────
    println!(
        "Summary: {} added, {} removed, {} modified, {} identical",
        only_in_2.len(),
        only_in_1.len(),
        modified.len(),
        identical.len()
    );

    Ok(())
}

fn format_size(bytes: usize) -> String {
    if bytes >= 1024 {
        format!("{} KB", bytes / 1024)
    } else {
        format!("{} B", bytes)
    }
}
