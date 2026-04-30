use crate::cli::OutputFormat;
use crate::dsk::Dsk;
use crate::utils;
use anyhow::Result;
use colored::*;
use std::path::Path;

pub fn execute<P: AsRef<Path>>(image: P, format: OutputFormat) -> Result<()> {
    let dsk = Dsk::open(image)?;
    let catalog = dsk.catalog()?;

    match format {
        OutputFormat::Table => print_table(&catalog, &dsk),
        OutputFormat::Json => print_json(&catalog),
        OutputFormat::Csv => print_csv(&catalog),
        OutputFormat::Simple => print_simple(&catalog),
    }

    Ok(())
}

fn print_table(catalog: &crate::dsk::Catalog, dsk: &Dsk) {
    println!();
    println!("{} disk.dsk", "📀 DSK Image:".bright_cyan().bold());
    println!(
        "   Tracks: {} | Sectors: {} | Format: {}",
        dsk.header().tracks,
        9, // TODO: get from track info
        "DATA".bright_green()
    );
    println!(
        "   Used: {} / {} ({}%)",
        utils::format_size(catalog.total_size),
        utils::format_size(catalog.total_size + catalog.free_space),
        (catalog.total_size * 100) / (catalog.total_size + catalog.free_space)
    );
    println!();

    // Table header
    println!("┌{:─<14}┬{:─<8}┬{:─<10}┬{:─<8}┐", "", "", "", "");
    println!(
        "│ {:<12} │ {:<6} │ {:<8} │ {:<6} │",
        "Name".bold(),
        "Type".bold(),
        "Size".bold(),
        "Attrs".bold()
    );
    println!("├{:─<14}┼{:─<8}┼{:─<10}┼{:─<8}┤", "", "", "", "");

    // Table rows
    for entry in &catalog.entries {
        let attrs = format!(
            "{}{}",
            if entry.read_only { "RO " } else { "" },
            if entry.system { "SYS" } else { "" }
        );

        println!(
            "│ {:<12} │ {:<6} │ {:<8} │ {:<6} │",
            entry.name.bright_white(),
            entry.file_type.yellow(),
            utils::format_size(entry.size),
            attrs.bright_red()
        );
    }

    println!("└{:─<14}┴{:─<8}┴{:─<10}┴{:─<8}┘", "", "", "", "");
    println!();
    println!(
        "{} files, {} total",
        catalog.entries.len(),
        utils::format_size(catalog.total_size)
    );
    println!();
}

/// Escape a string for embedding inside a JSON string literal.
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"'  => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => out.push(c),
        }
    }
    out
}

fn print_json(catalog: &crate::dsk::Catalog) {
    println!("{{");
    println!("  \"files\": [");
    for (i, entry) in catalog.entries.iter().enumerate() {
        println!("    {{");
        println!("      \"name\": \"{}\",", json_escape(&entry.name));
        println!("      \"size\": {},", entry.size);
        println!("      \"type\": \"{}\",", json_escape(&entry.file_type));
        println!("      \"read_only\": {},", entry.read_only);
        if let Some(load) = entry.load_address {
            println!("      \"load_address\": {},", load);
        }
        if let Some(exec) = entry.exec_address {
            println!("      \"exec_address\": {},", exec);
        }
        println!("      \"system\": {},", entry.system);
        println!("      \"user\": {}", entry.user);
        print!("    }}");
        if i < catalog.entries.len() - 1 {
            println!(",");
        } else {
            println!();
        }
    }
    println!("  ],");
    println!("  \"total_size\": {},", catalog.total_size);
    println!("  \"free_space\": {}", catalog.free_space);
    println!("}}");
}

fn print_csv(catalog: &crate::dsk::Catalog) {
    println!("name,size,type,read_only,system");
    for entry in &catalog.entries {
        println!(
            "{},{},{},{},{}",
            json_escape(&entry.name),
            entry.size,
            json_escape(&entry.file_type),
            entry.read_only,
            entry.system
        );
    }
}

fn print_simple(catalog: &crate::dsk::Catalog) {
    for entry in &catalog.entries {
        println!("{}", entry.name);
    }
}
