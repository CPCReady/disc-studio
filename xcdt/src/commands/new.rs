use std::path::Path;

use crate::error::Result;
use crate::tzx::write_file;
use super::write_common::{WriteOptions, build_tzx};

/// `xcdt new <input> <output.cdt>` — Create a new CDT file from a binary.
pub fn run(input: &Path, output: &Path, opts: &WriteOptions) -> Result<()> {
    let tzx = build_tzx(input, opts, true)?;
    write_file(&tzx, output)?;
    eprintln!(
        "Created {} ({} blocks)",
        output.display(),
        tzx.blocks.len()
    );
    Ok(())
}
