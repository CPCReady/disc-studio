use std::path::Path;

use crate::error::Result;
use crate::tzx::read_file;

/// `xcdt info <file.cdt>` — Show global metadata for a CDT/TZX file.
pub fn run(path: &Path, json: bool) -> Result<()> {
    let metadata = std::fs::metadata(path)?;
    let tzx = read_file(path)?;

    // Count block types
    let mut pauses = 0usize;
    let mut turbos = 0usize;
    let mut standards = 0usize;
    let mut pure_datas = 0usize;
    let mut unknowns = 0usize;
    let mut total_pause_ms: u64 = 0;
    let mut total_data_bytes: u64 = 0;

    for block in &tzx.blocks {
        match block {
            crate::tzx::TzxBlock::Pause(ms) => {
                pauses += 1;
                total_pause_ms += *ms as u64;
            }
            crate::tzx::TzxBlock::Turbo(b) => {
                turbos += 1;
                total_data_bytes += b.data.len() as u64;
                total_pause_ms += b.header.pause_ms as u64;
            }
            crate::tzx::TzxBlock::Standard(b) => {
                standards += 1;
                total_data_bytes += b.data.len() as u64;
                total_pause_ms += b.pause_ms as u64;
            }
            crate::tzx::TzxBlock::PureData(b) => {
                pure_datas += 1;
                total_data_bytes += b.data.len() as u64;
                total_pause_ms += b.pause_ms as u64;
            }
            crate::tzx::TzxBlock::Unknown { raw, .. } => {
                unknowns += 1;
                total_data_bytes += raw.len() as u64;
            }
        }
    }

    let data_secs = total_data_bytes as f64 / 1024.0;
    let total_secs = total_pause_ms as f64 / 1000.0 + data_secs;

    if json {
        let out = serde_json::json!({
            "file": path.display().to_string(),
            "size_bytes": metadata.len(),
            "version": format!("{}.{}", tzx.version_major, tzx.version_minor),
            "block_count": tzx.blocks.len(),
            "pause_blocks": pauses,
            "turbo_blocks": turbos,
            "standard_blocks": standards,
            "pure_data_blocks": pure_datas,
            "unknown_blocks": unknowns,
            "total_data_bytes": total_data_bytes,
            "total_pause_ms": total_pause_ms,
            "estimated_duration_secs": (total_secs as u64)
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        println!("File   : {}", path.display());
        println!("Size   : {} bytes", metadata.len());
        println!("Version: {}.{}", tzx.version_major, tzx.version_minor);
        println!("Blocks : {}", tzx.blocks.len());
        println!();
        if pauses > 0    { println!("  PAUSE blocks    : {}", pauses); }
        if turbos > 0    { println!("  TURBO blocks    : {}", turbos); }
        if standards > 0 { println!("  STANDARD blocks : {}", standards); }
        if pure_datas > 0 { println!("  PURE DATA blocks: {}", pure_datas); }
        if unknowns > 0  { println!("  UNKNOWN blocks  : {}", unknowns); }
        println!();
        println!("Total data payload : {} bytes", total_data_bytes);
        println!("Total pauses       : {} ms ({:.1}s)", total_pause_ms, total_pause_ms as f64 / 1000.0);
        println!("Estimated duration : ~{:.0}s ({:.1} min)", total_secs, total_secs / 60.0);
    }

    Ok(())
}
