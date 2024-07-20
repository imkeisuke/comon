use std::path::PathBuf;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[clap(
    version, author, about,
    arg_required_else_help = true,
)]
//コマンドラインツールのオプション解析

pub struct CliOpts {
  //MODE
  #[arg(short,long,default_value_t = RunMode::Auto, value_name = "MODE", required = false, ignore_case = true, value_enum, help = "Mode of operation.")]
  pub mode: RunMode,
  //抽出結果の保存先
  #[arg(short, long, default_value = ".", value_name = "DEST", required = false, help = "Destination of the extraction results.")]
  pub dest: Option<PathBuf>,
  #[arg(short, long, default_value = "Comon.zip", value_name = "OUTPUT", required = false, help = "Output file for the archive.")]
  pub output: Option<PathBuf>,
  #[arg(short, long = "no-recursive", help = "No reursiven mode.", default_value_t =false)]
  pub no_recursive: bool,
  #[arg(short,long, help = "Display verbose output.", default_value_t = false)]
  pub verbose: bool,
  #[arg(value_name = "ARGUMENTS", help = "List of files or direstories to be processed.", long_help = "extract mode: archive files to be extracted. archive mode: files to be archived. auto mode: if the arguments have archive files, it will extract them. Otherwise, it will archive the files.")]
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
