use clap::{Parser, Subcommand, ValueEnum};
use clap_complete::Shell;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "xdsk")]
#[command(author, version, about, long_about = None)]
#[command(
    about = "Modern DSK image manipulation tool for Amstrad CPC",
    long_about = "A modern, fast, and user-friendly tool for working with Amstrad CPC DSK disk images.\n\
                  Supports listing, importing, exporting, and viewing files in DSK format."
)]
pub struct Cli {
    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List files in a DSK image
    #[command(alias = "ls")]
    List {
        /// DSK image file
        image: PathBuf,

        /// Output format
        #[arg(short, long, value_enum, default_value = "table")]
        format: OutputFormat,
    },

    /// Import files into a DSK image
    #[command(alias = "add")]
    Import {
        /// DSK image file
        image: PathBuf,

        /// Files to import
        files: Vec<PathBuf>,

        #[command(flatten)]
        options: ImportOptions,
    },

    /// Export files from a DSK image
    #[command(alias = "get")]
    Export {
        /// DSK image file
        image: PathBuf,

        /// Files to export (supports wildcards)
        files: Vec<String>,

        /// Output directory
        #[arg(short, long)]
        output: Option<PathBuf>,

        #[command(flatten)]
        options: ExportOptions,
    },

    /// Remove files from a DSK image
    #[command(alias = "rm")]
    Remove {
        /// DSK image file
        image: PathBuf,

        /// Files to remove
        files: Vec<String>,

        /// Force removal without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Create a new DSK image
    #[command(alias = "new")]
    Create {
        /// DSK image file to create
        image: PathBuf,

        #[command(flatten)]
        options: CreateOptions,
    },

    /// View file content from a DSK image
    View {
        /// DSK image file
        image: PathBuf,

        /// File to view
        file: String,

        /// View format
        #[arg(short, long, value_enum, default_value = "auto")]
        format: ViewFormat,
    },

    /// Check DSK image integrity
    Check {
        /// DSK image file
        image: PathBuf,
    },

    /// Show detailed DSK image information
    Info {
        /// DSK image file
        image: PathBuf,
    },

    /// Change user number of a file in a DSK image
    Chuser {
        /// DSK image file
        image: PathBuf,

        /// File name
        file: String,

        /// New user number (0-15)
        #[arg(value_parser = clap::value_parser!(u8).range(0..=15))]
        user: u8,
    },

    /// Set file attributes in a DSK image
    Attr {
        /// DSK image file
        image: PathBuf,

        /// File name
        file: String,

        /// Set read-only attribute
        #[arg(long, conflicts_with = "no_read_only")]
        read_only: bool,

        /// Clear read-only attribute
        #[arg(long, conflicts_with = "read_only")]
        no_read_only: bool,

        /// Set system attribute
        #[arg(long, conflicts_with = "no_system")]
        system: bool,

        /// Clear system attribute
        #[arg(long, conflicts_with = "system")]
        no_system: bool,
    },

    /// Compare two DSK images
    Diff {
        /// First DSK image
        image1: PathBuf,

        /// Second DSK image
        image2: PathBuf,
    },

    /// Copy files between two DSK images
    #[command(alias = "cp")]
    Copy {
        /// Source DSK image
        src: PathBuf,

        /// Destination DSK image
        dst: PathBuf,

        /// Files to copy — supports wildcards (default: all files)
        files: Vec<String>,

        /// Overwrite if file already exists in destination
        #[arg(short, long)]
        force: bool,
    },

    /// Generate shell completion scripts
    #[command(hide = true)]
    Completions {
        /// Target shell
        shell: Shell,
    },

    /// Generate man pages for all subcommands
    #[command(hide = true)]
    Mangen {
        /// Output directory for man pages (created if it doesn't exist)
        #[arg(default_value = "man")]
        output_dir: PathBuf,
    },
}

#[derive(Clone, ValueEnum)]
pub enum OutputFormat {
    /// Human-readable table
    Table,
    /// JSON output
    Json,
    /// CSV output
    Csv,
    /// Simple list
    Simple,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum ViewFormat {
    /// Auto-detect format
    Auto,
    /// BASIC listing
    Basic,
    /// Hexadecimal dump
    Hex,
    /// ASCII text
    Ascii,
    /// Z80 disassembly
    Disasm,
}

#[derive(Parser)]
pub struct ImportOptions {
    /// File type (auto-detect if not specified)
    #[arg(short = 't', long, value_enum)]
    pub file_type: Option<FileType>,

    /// Load address (hex, e.g., 0x4000)
    #[arg(short = 'c', long, value_parser = parse_hex)]
    pub load: Option<u16>,

    /// Execution address (hex, e.g., 0xC000)
    #[arg(short = 'e', long, value_parser = parse_hex)]
    pub exec: Option<u16>,

    /// User number (0-15)
    #[arg(short, long, default_value = "0")]
    pub user: u8,

    /// Mark as read-only
    #[arg(short = 'o', long)]
    pub read_only: bool,

    /// Mark as system file
    #[arg(short, long)]
    pub system: bool,

    /// Force overwrite if file exists
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Parser)]
pub struct ExportOptions {
    /// Strip AMSDOS header
    #[arg(long)]
    pub strip_header: bool,
}

#[derive(Parser)]
pub struct CreateOptions {
    /// Number of tracks
    #[arg(long, default_value = "40")]
    pub tracks: u8,

    /// Number of sectors per track
    #[arg(long, default_value = "9")]
    pub sectors: u8,

    /// Force overwrite if file exists
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Clone, ValueEnum)]
pub enum FileType {
    /// ASCII text file
    Ascii,
    /// Binary file
    Binary,
    /// Raw file (no AMSDOS header)
    Raw,
}

fn parse_hex(s: &str) -> Result<u16, std::num::ParseIntError> {
    let s = s.trim_start_matches("0x").trim_start_matches("0X");
    u16::from_str_radix(s, 16)
}
