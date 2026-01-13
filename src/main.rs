use std::{fs, path::{Path, PathBuf}};
use clap::Parser;
use anyhow::{Context, Result};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(
    author,
    version,
    about = "Remove emojis from markdown files",
    long_about = "A CLI tool to strip emojis from markdown files. \
                  Can process single files or recursively scan directories."
)]
struct Cli {
    /// Path to a markdown file or directory containing .md files
    #[arg(short, long, value_name = "PATH")]
    path: PathBuf,

    /// Recursively process all .md files in the directory (replaces files in-place)
    #[arg(short, long)]
    recursive: bool,

    /// Output file path (only works with single file mode, ignored with --recursive)
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Show detailed processing information
    #[arg(short, long)]
    verbose: bool,

    /// Preview changes without modifying files
    #[arg(short = 'd', long)]
    dry_run: bool,

    /// Create backup files (.bak) before modifying (only with --recursive)
    #[arg(short, long)]
    backup: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.recursive {
        if !args.path.is_dir() {
            return Err(anyhow::anyhow!("Path must be a directory when using --recursive"));
        }
        process_directory(&args)?;
    } else {
        process_file(&args)?;
    }

    Ok(())
}

fn remove_emojis(content: &str) -> String {
    let re = Regex::new(r"\p{Emoji_Presentation}").unwrap();
    re.replace_all(content, "").to_string()
}

fn process_file(args: &Cli) -> Result<()> {
    let content = fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let cleaned_content = remove_emojis(&content);

    if args.dry_run {
        println!("[DRY RUN] Would process: {}", args.path.display());
        if args.verbose {
            println!("Original length: {} bytes", content.len());
            println!("Cleaned length: {} bytes", cleaned_content.len());
        }
        return Ok(());
    }

    match &args.output {
        Some(path) => {
            fs::write(path, cleaned_content.as_bytes())
                .with_context(|| format!("Could not write to file `{}`", path.display()))?;
            if args.verbose {
                println!("Successfully stripped emojis and saved to {}", path.display());
            }
        }
        None => {
            print!("{}", cleaned_content);
        }
    }

    Ok(())
}

fn process_file_in_place(file_path: &Path, args: &Cli) -> Result<()> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Could not read file `{}`", file_path.display()))?;

    let cleaned_content = remove_emojis(&content);

    if args.dry_run {
        if args.verbose {
            println!("[DRY RUN] Would process: {} ({} -> {} bytes)", 
                     file_path.display(), content.len(), cleaned_content.len());
        } else {
            println!("[DRY RUN] Would process: {}", file_path.display());
        }
        return Ok(());
    }

    // Create backup if requested
    if args.backup {
        let backup_path = file_path.with_extension("md.bak");
        fs::copy(file_path, &backup_path)
            .with_context(|| format!("Could not create backup at `{}`", backup_path.display()))?;
        if args.verbose {
            println!("Created backup: {}", backup_path.display());
        }
    }

    fs::write(file_path, cleaned_content)
        .with_context(|| format!("Could not write to file `{}`", file_path.display()))?;

    Ok(())
}

fn process_directory(args: &Cli) -> Result<()> {
    let mut processed = 0;
    let mut errors = 0;

    if args.verbose || args.dry_run {
        println!("Scanning directory: {}\n", args.path.display());
    }

    for entry in WalkDir::new(&args.path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension() == Some(std::ffi::OsStr::new("md")) {
            match process_file_in_place(entry.path(), args) {
                Ok(_) => {
                    if !args.dry_run && args.verbose {
                        println!("✓ Processed: {}", entry.path().display());
                    }
                    processed += 1;
                }
                Err(e) => {
                    eprintln!("✗ Error processing {}: {}", entry.path().display(), e);
                    errors += 1;
                }
            }
        }
    }

    println!("\nCompleted: {} files processed, {} errors", processed, errors);
    Ok(())
}