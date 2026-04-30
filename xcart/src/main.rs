// MIT License — Copyright (c) Destroyer 2026.
//
// xcart — Convert Amstrad CPC DSK disk images to GX-4000 cartridges (.cpr)

use std::path::PathBuf;
use std::process;

use clap::{Parser, Subcommand};

mod commands;
mod cpr;
mod dsk;
mod error;
mod roms;

// ─────────────────────────────────────────────────────────────────────────────
// CLI definition
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Parser)]
#[command(
    name = "xcart",
    version = env!("CARGO_PKG_VERSION"),
    author,
    about = "Convert Amstrad CPC DSK disk images to GX-4000 cartridges (.cpr)",
    long_about = "\
xcart converts Amstrad CPC DSK disk images into GX-4000 CPR cartridge files.

The OS, BASIC and AMSDOS ROMs are embedded in this binary and patched at
runtime to configure disk format and optional BASIC autostart.

Examples:
  xcart create game.dsk game.cpr
  xcart create game.dsk game.cpr -c 'run\"disc\"'
  xcart check  game.cpr
  xcart info   game.dsk
  xcart list   game.cpr -v
  xcart extract game.cpr 0 os.rom
  xcart dump   game.dsk sectors.bin",
    propagate_version = true,
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a CPR cartridge from a DSK disk image
    ///
    /// Embeds OS + BASIC + AMSDOS ROMs, patches AMSDOS for disk format and
    /// optional autostart, then packs the disk's sector data as data chunks.
    Create {
        /// Input DSK disk image
        input: PathBuf,

        /// Output CPR cartridge file
        output: PathBuf,

        /// BASIC autostart command (max 16 chars), e.g. `run"disc"` or `|cpm`
        #[arg(short, long, value_name = "COMMAND")]
        command: Option<String>,
    },

    /// Verify the structure of a CPR cartridge file
    ///
    /// Validates RIFF/AMS! header, chunk tags, chunk sizes and data integrity.
    Check {
        /// CPR cartridge file to check
        input: PathBuf,
    },

    /// Dump raw sector data from a DSK to a binary file
    ///
    /// Concatenates all sectors in track/identifier order without any headers.
    /// Useful for manual patching before re-assembling a cartridge.
    Dump {
        /// Input DSK disk image
        input: PathBuf,

        /// Output binary file
        output: PathBuf,
    },

    /// Display metadata for a DSK disk image or CPR cartridge
    ///
    /// Auto-detects file type from extension or magic bytes.
    Info {
        /// DSK or CPR file to inspect
        input: PathBuf,
    },

    /// List chunks in a CPR cartridge
    ///
    /// Prints a formatted table of all chunk indices, tags, sizes and descriptions.
    List {
        /// CPR cartridge file
        input: PathBuf,

        /// Show a hex preview of the first 16 bytes of each chunk
        #[arg(short, long)]
        verbose: bool,
    },

    /// Extract a single chunk from a CPR cartridge to a file
    ///
    /// Chunk indices: 0=OS ROM, 1=BASIC ROM, 2=AMSDOS ROM, 3+=disk data
    Extract {
        /// Input CPR cartridge file
        input: PathBuf,

        /// Chunk index (0-based)
        chunk: usize,

        /// Output file
        output: PathBuf,
    },
}

// ─────────────────────────────────────────────────────────────────────────────
// Entry point
// ─────────────────────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Create {
            input,
            output,
            command,
        } => commands::create::run(input, output, command.as_deref()),

        Commands::Check { input } => commands::check::run(input),

        Commands::Dump { input, output } => commands::dump::run(input, output),

        Commands::Info { input } => commands::info::run(input),

        Commands::List { input, verbose } => commands::list::run(input, *verbose),

        Commands::Extract {
            input,
            chunk,
            output,
        } => commands::extract::run(input, *chunk, output),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
