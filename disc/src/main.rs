mod amsdos;
mod cli;
mod commands;
mod dsk;
mod error;
mod utils;
mod viewers;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser};
use clap_complete::generate;
use clap_mangen::Man;
use cli::{Cli, Commands};
use colored::*;
use std::fs;
use std::path::Path;

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialise logger: --verbose → Debug, otherwise Warn
    env_logger::Builder::new()
        .filter_level(if cli.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Warn
        })
        .init();

    // Set color mode based on terminal support
    if !cli.no_color {
        colored::control::set_override(true);
    }

    let result = match cli.command {
        Commands::List { image, format } => commands::list::execute(&image, format),
        Commands::Import {
            image,
            files,
            options,
        } => commands::import::execute(&image, files, options),
        Commands::Export {
            image,
            files,
            output,
            options,
        } => commands::export::execute(&image, files, output, options),
        Commands::Remove {
            image,
            files,
            force,
        } => commands::remove::execute(&image, files, force),
        Commands::Create { image, options } => commands::create::execute(&image, options),
        Commands::View {
            image,
            file,
            format,
        } => commands::view::execute(&image, &file, format),
        Commands::Check { image } => commands::check::execute(&image),
        Commands::Info { image } => commands::info::execute(&image),
        Commands::Diff { image1, image2 } => commands::diff::execute(&image1, &image2),
        Commands::Copy {
            src,
            dst,
            files,
            force,
        } => commands::copy::execute(&src, &dst, files, force),
        Commands::Completions { shell } => {
            generate(shell, &mut Cli::command(), "disc", &mut std::io::stdout());
            return Ok(());
        }
        Commands::Mangen { output_dir } => {
            generate_man_pages(&output_dir)?;
            return Ok(());
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }

    Ok(())
}

/// Generates man pages (section 1) for `disc` and each of its subcommands.
///
/// The main page is written as `disc.1`.
/// Each subcommand page is written as `disc-<subcommand>.1`.
fn generate_man_pages(output_dir: &Path) -> Result<()> {
    fs::create_dir_all(output_dir).with_context(|| {
        format!(
            "Failed to create output directory: {}",
            output_dir.display()
        )
    })?;

    let cmd = Cli::command();

    // ── Main page ────────────────────────────────────────────────────────────
    let mut buf = Vec::<u8>::new();
    Man::new(cmd.clone())
        .render(&mut buf)
        .context("Failed to render main man page")?;
    let main_path = output_dir.join("disc.1");
    fs::write(&main_path, &buf)
        .with_context(|| format!("Failed to write {}", main_path.display()))?;
    println!("  wrote {}", main_path.display());

    // ── Subcommand pages ─────────────────────────────────────────────────────
    for sub in cmd.get_subcommands() {
        // Skip hidden utility subcommands (completions, mangen)
        if sub.is_hide_set() {
            continue;
        }

        let sub_name = sub.get_name();

        let mut buf = Vec::<u8>::new();
        Man::new(sub.clone())
            .render(&mut buf)
            .with_context(|| format!("Failed to render man page for disc-{}", sub_name))?;

        // File is stored as disc-<subcommand>.1 so man(1) finds it as "man disc-list", etc.
        let out_path = output_dir.join(format!("disc-{}.1", sub_name));
        fs::write(&out_path, &buf)
            .with_context(|| format!("Failed to write {}", out_path.display()))?;
        println!("  wrote {}", out_path.display());
    }

    println!(
        "{} Man pages written to {}",
        "✓".bright_green().bold(),
        output_dir.display()
    );
    Ok(())
}
