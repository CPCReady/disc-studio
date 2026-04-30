use std::path::Path;

use crate::crc;
use crate::error::Result;
use crate::timing::{SYNC_HEADER, SYNC_DATA};
use crate::tzx::{read_file, TzxBlock};

/// `xcdt check <file.cdt>` — Validate CDT/TZX structure and CRC integrity.
pub fn run(path: &Path, json: bool) -> Result<()> {
    let tzx = read_file(path)?;
    let mut errors = 0usize;
    let mut ok = 0usize;

    struct BlockResult {
        index: usize,
        block_type: String,
        status: String,
        detail: String,
    }
    let mut results: Vec<BlockResult> = Vec::new();

    if !json {
        println!("Checking: {}", path.display());
        println!("Version {}.{}", tzx.version_major, tzx.version_minor);
        println!();
    }

    for (i, block) in tzx.blocks.iter().enumerate() {
        let idx = i + 1;
        match block {
            TzxBlock::Turbo(b) => {
                if b.data.is_empty() {
                    let detail = "EMPTY".to_string();
                    if !json { println!("  Block {:>3} [TURBO]: {}", idx, detail); }
                    results.push(BlockResult { index: idx, block_type: "TURBO".into(), status: "error".into(), detail });
                    errors += 1;
                    continue;
                }
                let sync = b.data[0];
                let payload = &b.data[1..];
                let chunk_records = payload.len() / 258;
                let mut chunk_errors = 0usize;

                for c in 0..chunk_records {
                    let start = c * 258;
                    let chunk = &payload[start..start + 256];
                    let expected_crc = crc::compute_inverted(chunk);
                    let actual_crc = &payload[start + 256..start + 258];
                    if expected_crc != actual_crc {
                        chunk_errors += 1;
                        errors += 1;
                        let detail = format!("chunk {} CRC ERROR (expected {:02X}{:02X}, got {:02X}{:02X})",
                            c + 1, expected_crc[0], expected_crc[1], actual_crc[0], actual_crc[1]);
                        if !json { println!("  Block {:>3} [TURBO] {}", idx, detail); }
                        results.push(BlockResult { index: idx, block_type: "TURBO".into(), status: "error".into(), detail });
                    }
                }

                let sync_str = match sync {
                    s if s == SYNC_HEADER => "HDR",
                    s if s == SYNC_DATA => "DATA",
                    _ => "?",
                };

                if chunk_errors == 0 {
                    ok += 1;
                    let detail = format!("{} chunks, {} bytes, pause {}ms",
                        chunk_records, chunk_records * 256, b.header.pause_ms);
                    if !json { println!("  Block {:>3} [TURBO/{:4}]: OK ({})", idx, sync_str, detail); }
                    results.push(BlockResult { index: idx, block_type: format!("TURBO/{}", sync_str), status: "ok".into(), detail });
                }
            }

            TzxBlock::Standard(b) => {
                if b.data.len() < 2 {
                    let detail = "TOO SHORT".to_string();
                    if !json { println!("  Block {:>3} [STANDARD]: {}", idx, detail); }
                    results.push(BlockResult { index: idx, block_type: "STANDARD".into(), status: "error".into(), detail });
                    errors += 1;
                    continue;
                }
                let check = b.data[..b.data.len() - 1].iter().fold(0u8, |acc, &x| acc ^ x);
                let stored = *b.data.last().unwrap();
                if check != stored {
                    errors += 1;
                    let detail = format!("CHECKSUM ERROR (expected 0x{:02X}, got 0x{:02X})", check, stored);
                    if !json { println!("  Block {:>3} [STANDARD]: {}", idx, detail); }
                    results.push(BlockResult { index: idx, block_type: "STANDARD".into(), status: "error".into(), detail });
                } else {
                    ok += 1;
                    let detail = format!("{} bytes, pause {}ms", b.data.len(), b.pause_ms);
                    if !json { println!("  Block {:>3} [STANDARD]: OK ({})", idx, detail); }
                    results.push(BlockResult { index: idx, block_type: "STANDARD".into(), status: "ok".into(), detail });
                }
            }

            TzxBlock::Pause(ms) => {
                ok += 1;
                let detail = format!("{} ms", ms);
                if !json { println!("  Block {:>3} [PAUSE]: {}", idx, detail); }
                results.push(BlockResult { index: idx, block_type: "PAUSE".into(), status: "ok".into(), detail });
            }

            TzxBlock::PureData(b) => {
                ok += 1;
                let detail = format!("{} bytes bitstream, pause {}ms", b.data.len(), b.pause_ms);
                if !json { println!("  Block {:>3} [PURE DATA]: {}", idx, detail); }
                results.push(BlockResult { index: idx, block_type: "PURE DATA".into(), status: "ok".into(), detail });
            }

            TzxBlock::Unknown { id, raw } => {
                let detail = format!("{} bytes", raw.len());
                if !json { println!("  Block {:>3} [UNKNOWN 0x{:02X}]: {}", idx, id, detail); }
                results.push(BlockResult { index: idx, block_type: format!("UNKNOWN 0x{:02X}", id), status: "unknown".into(), detail });
            }
        }
    }

    if json {
        let blocks_json: Vec<serde_json::Value> = results.iter().map(|r| {
            serde_json::json!({
                "index": r.index,
                "type": r.block_type,
                "status": r.status,
                "detail": r.detail
            })
        }).collect();
        let out = serde_json::json!({
            "file": path.display().to_string(),
            "version": format!("{}.{}", tzx.version_major, tzx.version_minor),
            "ok": errors == 0,
            "errors": errors,
            "blocks_verified": ok,
            "blocks": blocks_json
        });
        println!("{}", serde_json::to_string_pretty(&out).unwrap());
    } else {
        println!();
        if errors == 0 {
            println!("Result: OK ({} blocks verified)", ok);
        } else {
            println!("Result: {} ERROR(S) in {} blocks", errors, ok + errors);
        }
    }

    if errors > 0 {
        std::process::exit(1);
    }
    Ok(())
}
