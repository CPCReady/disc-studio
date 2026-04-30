use std::path::Path;

use crate::error::Result;
use crate::tzx::{read_file, TzxBlock};

/// `xcdt list <file.cdt>` — List all TZX blocks with metadata.
pub fn run(path: &Path, json: bool) -> Result<()> {
    let tzx = read_file(path)?;

    struct Row {
        index: usize,
        type_name: String,
        id: u8,
        data_size: usize,
        baud: Option<u32>,
        pause_ms: u16,
    }

    let rows: Vec<Row> = tzx.blocks.iter().enumerate().map(|(i, block)| {
        let baud = match block {
            TzxBlock::Turbo(b) => {
                let r = b.approx_baud();
                if r > 0 { Some(r) } else { None }
            }
            _ => None,
        };
        Row {
            index: i + 1,
            type_name: block.type_name().to_string(),
            id: block.id(),
            data_size: block.data_size(),
            baud,
            pause_ms: block.pause_ms(),
        }
    }).collect();

    if json {
        let blocks: Vec<serde_json::Value> = rows.iter().map(|r| {
            serde_json::json!({
                "index": r.index,
                "type": r.type_name,
                "id": format!("0x{:02X}", r.id),
                "data_size": r.data_size,
                "baud": r.baud,
                "pause_ms": r.pause_ms
            })
        }).collect();
        let out = serde_json::json!({
            "file": path.display().to_string(),
            "version": format!("{}.{}", tzx.version_major, tzx.version_minor),
            "block_count": rows.len(),
            "blocks": blocks
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        println!("CDT/TZX: {}", path.display());
        println!("Version {}.{}", tzx.version_major, tzx.version_minor);
        println!("Blocks: {}", rows.len());
        println!();
        println!(
            "{:<4}  {:<10}  {:<4}  {:>10}  {:>8}  {:>8}",
            "#", "Type", "ID", "DataSize", "Baud~", "Pause(ms)"
        );
        println!("{}", "-".repeat(55));
        for r in &rows {
            let baud_str = r.baud.map(|b| b.to_string()).unwrap_or_else(|| "-".to_string());
            println!(
                "{:<4}  {:<10}  0x{:02X}  {:>10}  {:>8}  {:>8}",
                r.index, r.type_name, r.id, r.data_size, baud_str, r.pause_ms,
            );
        }
    }

    Ok(())
}
