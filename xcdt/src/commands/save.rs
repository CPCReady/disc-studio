use std::path::Path;

use crate::error::Result;
use crate::tzx::append_file;
use super::write_common::{WriteOptions, build_tzx};

/// `xcdt save <input> <output.cdt>` — Append a file to an existing CDT.
pub fn run(input: &Path, output: &Path, opts: &WriteOptions) -> Result<()> {
    let tzx = build_tzx(input, opts, false)?;
    append_file(&tzx, output)?;
    eprintln!(
        "Appended {} blocks to {}",
        tzx.blocks.len(),
        output.display()
    );
    Ok(())
}
