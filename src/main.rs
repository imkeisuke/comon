mod archiver;
mod format;
mod tote_error;

use crate::archiver::{create_archiver, Archiver};
use crate::format::detect_format;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Comon")]
#[command(about = "A simple archiving utility", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Compress {
        #[arg(short, long)]
        format: String,
        #[arg(short, long)]
        src: PathBuf,
        #[arg(short, long)]
        dest: PathBuf,
    },
    Decompress {
        #[arg(short, long)]
        src: PathBuf,
        #[arg(short, long)]
        dest: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Compress { format, src, dest } => {
            let format = detect_format(format).unwrap();
            let archiver = create_archiver(format).unwrap();
            archiver.compress(src, dest).unwrap();
        }
        Commands::Decompress { src, dest } => {
            let format = detect_format(src.to_str().unwrap()).unwrap();
            let archiver = create_archiver(format).unwrap();
            archiver.decompress(src, dest).unwrap();
        }
    }
}
