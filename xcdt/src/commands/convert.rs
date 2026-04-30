use std::path::Path;

use crate::commands::write_common::{make_block, BlockType};
use crate::error::{Error, Result};
use crate::timing::TimingParams;
use crate::tzx::{read_file, write_file, TzxBlock};

/// `xcdt convert <input.cdt> <output.cdt> --to <type> [--block <idx>] [-b baud]`
///
/// Converts Turbo and Standard blocks to a different block encoding. Pause,
/// PureData and Unknown blocks are left unchanged.
///
/// `block_index` is 1-based (as shown by `xcdt list`). When `None`, all
/// convertible blocks are processed.
///
/// `baud` is used for the re-encoded Turbo/PureData timing parameters. When
/// the source is a Turbo block and no explicit baud was given, the block's own
/// approximate baud is used so the timing is preserved.
pub fn run(
    input: &Path,
    output: &Path,
    target: BlockType,
    block_index: Option<usize>,
    baud: Option<u32>,
) -> Result<()> {
    let mut tzx = read_file(input)?;
    let mut count = 0usize;

    for (idx, block) in tzx.blocks.iter_mut().enumerate() {
        let one_based = idx + 1;

        // If a specific block was requested, skip all others.
        if let Some(wanted) = block_index {
            if one_based != wanted {
                continue;
            }
        }

        // Decode the block into (sync_byte, raw_payload, pause_ms).
        // Only Turbo and Standard blocks are decodable; all others are skipped.
        let (sync, raw, pause_ms, src_baud) = match &*block {
            TzxBlock::Turbo(b) => match b.decode() {
                Some((s, d)) => (s, d, b.header.pause_ms, Some(b.approx_baud())),
                None => continue,
            },
            TzxBlock::Standard(b) => match b.decode() {
                Some((s, d)) => (s, d, b.pause_ms, None),
                None => continue,
            },
            _ => continue,
        };

        // Choose timing params: explicit baud > source baud > default 2000.
        let effective_baud = baud
            .or(src_baud)
            .unwrap_or(2000)
            .clamp(1000, 6000);
        let timing = TimingParams::for_baud(effective_baud);

        *block = make_block(target, &timing, sync, &raw, pause_ms);
        count += 1;
    }

    // If a specific block index was requested but nothing was converted, error.
    if let Some(wanted) = block_index {
        if count == 0 {
            return Err(Error::Other(format!(
                "Block {} not found or is not a convertible type (TURBO or STANDARD required)",
                wanted
            )));
        }
    }

    write_file(&tzx, output)?;

    let type_label = match target {
        BlockType::Turbo => "TURBO",
        BlockType::PureData => "PURE_DATA",
        BlockType::Standard => "STANDARD",
    };

    eprintln!(
        "Converted {} block{} → {} in {}",
        count,
        if count == 1 { "" } else { "s" },
        type_label,
        output.display()
    );

    Ok(())
}
