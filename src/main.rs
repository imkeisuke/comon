use clap::Parser;
use cli::CliOpts;
use std::path::PathBuf;

mod archiver;
mod cli;
mod format;
mod tote_error;
mod verboser;

fn main() {
    let opts = CliOpts::parse();
    let archiver_opts = archiver::ArchiverOpts::new(&opts);

    match opts.mode {
        cli::RunMode::Archive => {
            let archiver = archiver::create_archiver(&archiver_opts.dest).unwrap();
            archiver.perform(&archiver_opts).unwrap();
        }
        cli::RunMode::Extract => {
            let archiver = archiver::create_archiver(&archiver_opts.dest).unwrap();
            archiver.perform(&archiver_opts).unwrap();
        }
        cli::RunMode::Auto => {
            // Implement auto-detection of mode based on file extension
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cli::RunMode;

    #[test]
    fn test_run() {
        let opts = CliOpts::parse_from(&[
            "comon_test",
            "-o", "test.zip",
            "src",
            "LICENSE",
            "README.md",
            "Cargo.toml"
        ]);

        assert_eq!(opts.mode, RunMode::Auto);
        assert_eq!(opts.output, Some(PathBuf::from("test.zip")));
        assert_eq!(opts.args.len(), 4);
        assert_eq!(opts.args, vec![
            PathBuf::from("src"),
            PathBuf::from("LICENSE"),
            PathBuf::from("README.md"),
            PathBuf::from("Cargo.toml")
        ]);
    }
}
