// MIT License - Copyright (c) 2026 Destroyer

use crate::amsdos::{has_header, AmsdosHeader};
use crate::cli::ViewFormat;
use crate::dsk::Dsk;
use crate::viewers::basic::is_tokenized;
use crate::viewers::{AsciiViewer, BasicViewer, DisasmViewer, HexViewer};
use anyhow::{Context, Result};
use colored::*;
use std::path::PathBuf;

pub fn execute(image: &PathBuf, file: &str, format: ViewFormat) -> Result<()> {
    let dsk = Dsk::open(image).context(format!("Failed to open DSK image: {}", image.display()))?;

    // Find file in catalog
    let catalog = dsk.catalog()?;
    let entry = catalog
        .entries
        .iter()
        .find(|e| e.name.to_uppercase() == file.to_uppercase());

    if entry.is_none() {
        anyhow::bail!("File not found in DSK: {}", file);
    }

    // Read file data
    let (file_data, amsdos) = read_file_data(&dsk, file)?;

    // Determine load address for disasm/hex
    let load_addr = amsdos.as_ref().map(|h| h.load_address).unwrap_or(0);

    // Auto-detect format if needed
    let actual_format = match format {
        ViewFormat::Auto => detect_format(&file_data, file, &amsdos),
        other => other,
    };

    // Print header
    println!();
    print!("{} {}", "📄".bright_cyan(), file.bright_white().bold());
    if let Some(ref h) = amsdos {
        let ftype = crate::amsdos::FileType::from_u8(h.file_type);
        print!("  [{}]", ftype.as_str().yellow());
        if h.load_address > 0 {
            print!(
                "  load: {}",
                format!("0x{:04X}", h.load_address).bright_blue()
            );
        }
        if h.entry_address > 0 {
            print!(
                "  exec: {}",
                format!("0x{:04X}", h.entry_address).bright_blue()
            );
        }
        print!("  {} bytes", h.logical_length);
    } else {
        print!("  {} bytes", file_data.len());
    }
    println!();
    println!("{}", "─".repeat(60).bright_black());
    println!();

    // Render
    let output = match actual_format {
        ViewFormat::Basic => BasicViewer::view(&file_data, true),
        ViewFormat::Hex => HexViewer::view(&file_data, Some(load_addr)),
        ViewFormat::Ascii => AsciiViewer::view(&file_data),
        ViewFormat::Disasm => DisasmViewer::view(&file_data, load_addr),
        ViewFormat::Auto => unreachable!(),
    };

    print!("{}", output);
    Ok(())
}

fn detect_format(data: &[u8], filename: &str, amsdos: &Option<AmsdosHeader>) -> ViewFormat {
    // Check AMSDOS header type first
    if let Some(h) = amsdos {
        return match h.file_type {
            0 | 1 => ViewFormat::Basic, // BASIC or BASIC protected
            2 | 3 => ViewFormat::Hex,   // Binary
            _ => ViewFormat::Ascii,
        };
    }

    // No header: detect by content
    if is_tokenized(data) {
        return ViewFormat::Basic;
    }

    // Check extension
    let ext = filename.rsplit('.').next().unwrap_or("").to_uppercase();
    match ext.as_str() {
        "BAS" => {
            // Only decode as tokenized BASIC if it actually is tokenized;
            // otherwise it is an ASCII-saved BASIC file
            if is_tokenized(data) {
                ViewFormat::Basic
            } else {
                ViewFormat::Ascii
            }
        }
        "BIN" | "COM" | "OBJ" => ViewFormat::Hex,
        "TXT" | "ASC" | "DOC" => ViewFormat::Ascii,
        _ => {
            // Heuristic: if mostly printable ASCII, show as text
            let printable = data
                .iter()
                .filter(|&&b| (0x20..0x7F).contains(&b) || b == b'\r' || b == b'\n')
                .count();
            if printable > data.len() * 8 / 10 {
                ViewFormat::Ascii
            } else {
                ViewFormat::Hex
            }
        }
    }
}

fn read_file_data(dsk: &Dsk, filename: &str) -> Result<(Vec<u8>, Option<AmsdosHeader>)> {
    let upper = filename.to_uppercase();
    let raw = dsk
        .read_file_data(&upper)?
        .ok_or_else(|| anyhow::anyhow!("File not found in DSK: {}", filename))?;

    if raw.len() >= 128 && has_header(&raw) {
        let amsdos = AmsdosHeader::from_bytes(&raw[..128]).ok();
        let mut payload = raw[128..].to_vec();

        if let Some(ref h) = amsdos {
            let logical = h.logical_length as usize;
            if logical > 0 && payload.len() > logical {
                payload.truncate(logical);
            }
        }

        return Ok((payload, amsdos));
    }

    Ok((raw, None))
}
