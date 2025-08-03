use clap::Parser;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Simple file sync utility
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // Source directory to sync from
    #[arg(short, long)]
    source: PathBuf,

    /// Destination directory (e.g., your 24TB HDD mount)
    #[arg(short, long)]
    destination: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let source = Path::new(&args.source);
    // let destination = Path::new(&args.destination);

    if !source.exists() || !source.is_dir() {
        anyhow::bail!("Source path doesn't exist or is not a directory");
    }

    // lets print what is in the source directory
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }

    Ok(())
}
