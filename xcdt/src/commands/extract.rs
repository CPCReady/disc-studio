use std::path::Path;

use crate::error::{Error, Result};
use crate::tzx::{read_file, TzxBlock};

/// `xcdt extract <file.cdt> <block_index> <output>` — Extract raw block data.
///
/// `block_index` is 1-based (as shown by `xcdt list`).
pub fn run(input: &Path, block_index: usize, output: &Path) -> Result<()> {
    if block_index == 0 {
        return Err(Error::Other("Block index must be 1 or greater".into()));
    }

    let tzx = read_file(input)?;
    let idx = block_index - 1;

    if idx >= tzx.blocks.len() {
        return Err(Error::Other(format!(
            "Block {} not found (file has {} blocks)",
            block_index,
            tzx.blocks.len()
        )));
    }

    let block = &tzx.blocks[idx];
    let data: &[u8] = match block {
        TzxBlock::Turbo(b) => &b.data,
        TzxBlock::Standard(b) => &b.data,
        TzxBlock::PureData(b) => &b.data,
        TzxBlock::Pause(_) => {
            return Err(Error::Other(format!(
                "Block {} is a PAUSE block — no data payload to extract",
                block_index
            )));
        }
        TzxBlock::Unknown { raw, .. } => raw,
    };

    std::fs::write(output, data)?;
    println!(
        "Extracted block {} ({}, {} bytes) → {}",
        block_index,
        block.type_name(),
        data.len(),
        output.display()
    );

    Ok(())
}
