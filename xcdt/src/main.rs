mod amsdos;
mod commands;
mod crc;
mod encoder;
mod error;
mod timing;
mod tzx;

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueEnum};
use error::Result;

// ── CLI definition ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum BlockTypeArg {
    /// Pure Data bitstream (ID 0x14)
    #[value(name = "0")]
    PureData,
    /// Turbo Loading (ID 0x11) — default
    #[value(name = "1")]
    Turbo,
    /// Standard Speed (ID 0x10)
    #[value(name = "2")]
    Standard,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum MethodArg {
    /// CPC blocks: 64B tape header + 2 KB data blocks (default)
    #[value(name = "0")]
    Blocks,
    /// Headerless: single continuous block, no tape header
    #[value(name = "1")]
    Headerless,
    /// Spectrum standard speed block
    #[value(name = "2")]
    Spectrum,
}

/// Shared write flags used by `new` and `save`.
#[derive(Debug, Parser)]
struct WriteFlags {
    /// Baud rate (1000-6000, default 2000)
    #[arg(short = 'b', long, default_value = "2000")]
    baud: u32,

    /// TZX block type: 0=Pure Data, 1=Turbo (default), 2=Standard
    #[arg(short = 't', long, value_enum, default_value = "1")]
    block_type: BlockTypeArg,

    /// Data method: 0=blocks (default), 1=headerless, 2=spectrum
    #[arg(short = 'm', long, value_enum, default_value = "0")]
    method: MethodArg,

    /// Rename file on tape (max 16 chars, uppercase)
    #[arg(short = 'r', long)]
    rename: Option<String>,

    /// Load address (hex: &XXXX, 0xXXXX, or decimal)
    #[arg(short = 'L', long)]
    load: Option<String>,

    /// Execution address (hex: &XXXX, 0xXXXX, or decimal)
    #[arg(short = 'X', long)]
    exec: Option<String>,

    /// File type: 0=BASIC, 1=Protected, 2=Binary (default)
    #[arg(short = 'F', long)]
    file_type: Option<u8>,

    /// Initial pause before first block in milliseconds (default 3000)
    #[arg(short = 'p', long, default_value = "3000")]
    pause_ms: u16,

    /// Add 1 ms pre-pause for emulators that need it
    #[arg(short = 'P', long)]
    buggy_emu: bool,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create a new CDT file from a binary
    New {
        /// Source binary file
        input: PathBuf,
        /// Output CDT file (created or overwritten)
        output: PathBuf,
        #[command(flatten)]
        flags: WriteFlags,
    },

    /// Append a binary to an existing CDT file
    Save {
        /// Source binary file
        input: PathBuf,
        /// Target CDT file (created if not present)
        output: PathBuf,
        #[command(flatten)]
        flags: WriteFlags,
    },

    /// List AMSDOS files stored in a CDT
    Cat {
        /// CDT/TZX file to read
        input: PathBuf,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// List all TZX blocks in a CDT
    List {
        /// CDT/TZX file to read
        input: PathBuf,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Validate CDT/TZX structure and CRC integrity
    Check {
        /// CDT/TZX file to validate
        input: PathBuf,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Show global metadata for a CDT/TZX file
    Info {
        /// CDT/TZX file to inspect
        input: PathBuf,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Extract the raw data payload of a specific block
    Extract {
        /// CDT/TZX file to read
        input: PathBuf,
        /// Block index (1-based, as shown by `xcdt list`)
        block: usize,
        /// Output file for the extracted data
        output: PathBuf,
    },

    /// Rename an AMSDOS file on tape (patches header block in-place)
    Rename {
        /// CDT/TZX file to modify
        input: PathBuf,
        /// Current filename on tape (case-insensitive, max 16 chars)
        old_name: String,
        /// New filename (uppercase, max 16 chars)
        new_name: String,
        /// Write output to a new file instead of modifying in-place
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
    },

    /// Convert blocks between encoding types (TURBO ↔ STANDARD ↔ PURE_DATA)
    Convert {
        /// Input CDT/TZX file
        input: PathBuf,
        /// Output CDT/TZX file
        output: PathBuf,
        /// Target block type: 0=Pure Data, 1=Turbo, 2=Standard
        #[arg(long = "to", value_enum)]
        target: BlockTypeArg,
        /// Convert only this block index (1-based); default: convert all
        #[arg(long)]
        block: Option<usize>,
        /// Baud rate for re-encoding (1000-6000); default: use source baud or 2000
        #[arg(short = 'b', long)]
        baud: Option<u32>,
    },
}

#[derive(Debug, Parser)]
#[command(
    name = "xcdt",
    version = env!("CARGO_PKG_VERSION"),
    about = "Create and inspect Amstrad CPC CDT/TZX tape images",
    long_about = concat!(
        "xcdt — Amstrad CPC CDT/TZX tape image tool\n\n",
        "COMMANDS:\n",
        "  new      Create a new CDT from a binary file\n",
        "  save     Append a binary file to an existing CDT\n",
        "  cat      List AMSDOS files on a CDT tape\n",
        "  list     List all TZX blocks in a CDT\n",
        "  check    Validate CDT structure and CRC\n",
        "  info     Show global file metadata\n",
        "  extract  Extract raw data from a specific block\n",
        "  rename   Rename an AMSDOS file on tape\n",
        "  convert  Convert blocks between encoding types\n",
    )
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn parse_address(s: &str) -> Result<u16> {
    let s = s.trim();
    let (hex, val) = if let Some(rest) = s.strip_prefix('&') {
        (true, rest)
    } else if let Some(rest) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        (true, rest)
    } else {
        (false, s)
    };

    if hex {
        u16::from_str_radix(val, 16).map_err(|_| {
            error::Error::Other(format!("Invalid hex address: {}", s))
        })
    } else {
        val.parse::<u16>().map_err(|_| {
            error::Error::Other(format!("Invalid address: {}", s))
        })
    }
}

fn build_write_opts(flags: &WriteFlags) -> Result<commands::WriteOptions> {
    if flags.baud < 1000 || flags.baud > 6000 {
        return Err(error::Error::Other(
            "Baud rate must be between 1000 and 6000".into(),
        ));
    }

    let block_type = match flags.block_type {
        BlockTypeArg::Turbo => commands::BlockType::Turbo,
        BlockTypeArg::PureData => commands::BlockType::PureData,
        BlockTypeArg::Standard => commands::BlockType::Standard,
    };
    let method = match flags.method {
        MethodArg::Blocks => commands::DataMethod::Blocks,
        MethodArg::Headerless => commands::DataMethod::Headerless,
        MethodArg::Spectrum => commands::DataMethod::Spectrum,
    };

    let load_address = flags
        .load
        .as_deref()
        .map(parse_address)
        .transpose()?;
    let exec_address = flags
        .exec
        .as_deref()
        .map(parse_address)
        .transpose()?;

    let tape_name = flags.rename.as_ref().map(|n| {
        let n = n.to_ascii_uppercase();
        n.chars().take(16).collect::<String>()
    });

    Ok(commands::WriteOptions {
        baud: flags.baud,
        block_type,
        method,
        tape_name,
        load_address,
        exec_address,
        file_type: flags.file_type,
        initial_pause_ms: flags.pause_ms,
        buggy_emu_pause: flags.buggy_emu,
    })
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::New { input, output, flags } => {
            build_write_opts(&flags).and_then(|opts| commands::new::run(&input, &output, &opts))
        }
        Commands::Save { input, output, flags } => {
            build_write_opts(&flags).and_then(|opts| commands::save::run(&input, &output, &opts))
        }
        Commands::Cat { input, json } => commands::cat::run(&input, json),
        Commands::List { input, json } => commands::list::run(&input, json),
        Commands::Check { input, json } => commands::check::run(&input, json),
        Commands::Info { input, json } => commands::info::run(&input, json),
        Commands::Extract { input, block, output } => {
            commands::extract::run(&input, block, &output)
        }
        Commands::Rename { input, old_name, new_name, output } => {
            commands::rename::run(&input, &old_name, &new_name, output.as_deref())
        }
        Commands::Convert { input, output, target, block, baud } => {
            let block_type = match target {
                BlockTypeArg::Turbo => commands::BlockType::Turbo,
                BlockTypeArg::PureData => commands::BlockType::PureData,
                BlockTypeArg::Standard => commands::BlockType::Standard,
            };
            if let Some(b) = baud {
                if b < 1000 || b > 6000 {
                    Err(error::Error::Other(
                        "Baud rate must be between 1000 and 6000".into(),
                    ))
                } else {
                    commands::convert::run(&input, &output, block_type, block, baud)
                }
            } else {
                commands::convert::run(&input, &output, block_type, block, baud)
            }
        }
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
