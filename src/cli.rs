use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(
    version, author, about,
    arg_required_else_help = true,
)]
pub struct CliOpts {
    #[arg(short, long, default_value_t = RunMode::Auto, value_enum, help = "Mode of operation.")]
    pub mode: RunMode,

    #[arg(short, long, default_value = ".", help = "Destination of the extraction results.")]
    pub dest: Option<PathBuf>,

    #[arg(short, long, default_value = "Comon.zip", help = "Output file for the archive.")]
    pub output: Option<PathBuf>,

    #[arg(short, long, help = "No recursive mode.", default_value_t = false)]
    pub no_recursive: bool,

    #[arg(short, long, help = "Display verbose output.", default_value_t = false)]
    pub verbose: bool,

    #[arg(help = "List of files or directories to be processed.")]
    pub args: Vec<PathBuf>,

    #[arg(short, long, help = "Extract to a directory named after the archive file.")]
    pub to_archive_name_dir: bool,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum RunMode {
    Auto,
    Archive,
    Extract,
}

#[derive(Debug)]
pub enum ToteError {
    IO(std::io::Error),
    UnknownFormat(String),
    FileExists(PathBuf),
}
