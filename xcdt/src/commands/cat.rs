use std::path::Path;

use crate::amsdos::{AmsdosHeader, TAPE_HEADER_SIZE};
use crate::error::Result;
use crate::timing::SYNC_HEADER;
use crate::tzx::{read_file, TzxBlock};

/// `xcdt cat <file.cdt>` — List files stored on a CDT tape.
pub fn run(path: &Path, json: bool) -> Result<()> {
    let tzx = read_file(path)?;

    struct Entry {
        index: usize,
        name: String,
        file_type: String,
        load: u16,
        exec: u16,
        size: u16,
        block: u8,
        first: bool,
        last: bool,
    }

    let mut entries: Vec<Entry> = Vec::new();

    for block in &tzx.blocks {
        let (sync, data) = match block {
            TzxBlock::Turbo(b) => match b.decode() {
                Some(pair) => pair,
                None => continue,
            },
            TzxBlock::Standard(b) => match b.decode() {
                Some(pair) => pair,
                None => continue,
            },
            _ => continue,
        };

        if sync != SYNC_HEADER || data.len() < TAPE_HEADER_SIZE {
            continue;
        }

        if let Some(hdr) = AmsdosHeader::from_bytes(&data[..TAPE_HEADER_SIZE]) {
            entries.push(Entry {
                index: entries.len() + 1,
                name: hdr.name_str(),
                file_type: hdr.type_str().to_string(),
                load: u16::from_le_bytes([data[21], data[22]]),
                exec: u16::from_le_bytes([data[26], data[27]]),
                size: u16::from_le_bytes([data[19], data[20]]),
                block: data[16],
                first: data[23] == 0xFF,
                last: data[17] == 0xFF,
            });
        }
    }

    if json {
        let files: Vec<serde_json::Value> = entries
            .iter()
            .map(|e| {
                serde_json::json!({
                    "index": e.index,
                    "name": e.name,
                    "type": e.file_type,
                    "load": format!("0x{:04X}", e.load),
                    "exec": format!("0x{:04X}", e.exec),
                    "size": e.size,
                    "block": e.block,
                    "first": e.first,
                    "last": e.last
                })
            })
            .collect();

        let out = serde_json::json!({
            "file": path.display().to_string(),
            "version": format!("{}.{}", tzx.version_major, tzx.version_minor),
            "count": entries.len(),
            "files": files
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        println!("CDT/TZX: {}", path.display());
        println!("Version {}.{}", tzx.version_major, tzx.version_minor);
        println!();
        println!(
            "{:<3}  {:<16}  {:<8}  {:<7}  {:<7}  {:<7}  {:<5}  {}",
            "#", "Name", "Type", "Load@", "Exec@", "Size", "Block", "First/Last"
        );
        println!("{}", "-".repeat(75));

        if entries.is_empty() {
            println!("  (no AMSDOS files found)");
        } else {
            for e in &entries {
                let first_last = match (e.first, e.last) {
                    (true, true) => "FIRST LAST",
                    (true, false) => "FIRST",
                    (false, true) => "LAST",
                    _ => "",
                };
                println!(
                    "{:<3}  {:<16}  {:<8}  &{:04X}   &{:04X}   {:>5}B  {:>5}  {}",
                    e.index, e.name, e.file_type, e.load, e.exec,
                    e.size, e.block, first_last,
                );
            }
        }
    }

    Ok(())
}
