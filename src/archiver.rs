use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};

use crate::archiver::tar::TarArchiver;
use crate::archiver::zip::ZipArchiver;
use crate::cli::{CliOpts};
use crate::format::{find_format, Format};
use crate::tote_error::ToteError;
use crate::verboser::{create_verboser, Verboser};

mod tar;
mod zip;

pub trait Archiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<(), ToteError>;
    fn format(&self) -> Format;
}

pub struct ArchiverOpts {
    pub dest: PathBuf,
    pub targets: Vec<PathBuf>,
    pub overwrite: bool,
    pub recursive: bool,
    pub v: Box<dyn Verboser>,
}

impl ArchiverOpts {
    pub fn new(opts: &CliOpts) -> Self {
        ArchiverOpts {
            dest: opts.output.clone().unwrap_or_else(|| PathBuf::from(".")),
            targets: opts.args.clone(),
            overwrite: false,
            recursive: !opts.no_recursive,
            v: create_verboser(opts.verbose),
        }
    }

    #[cfg(test)]
    pub fn create(
        dest: PathBuf,
        targets: Vec<PathBuf>,
        overwrite: bool,
        recursive: bool,
        verbose: bool,
    ) -> Self {
        ArchiverOpts {
            dest,
            targets,
            overwrite,
            recursive,
            v: create_verboser(verbose),
        }
    }

    pub fn targets(&self) -> Vec<PathBuf> {
        self.targets.clone()
    }

    pub fn dest_path(&self) -> PathBuf {
        self.dest.clone()
    }

    pub fn destination(&self) -> Result<File, std::io::Error> {
        let p = self.dest.as_path();
        if p.is_file() && p.exists() && !self.overwrite {
            return Err(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                format!("File exists: {}", self.dest.display()),
            ));
        }
        if let Some(parent) = p.parent() {
            if !parent.exists() {
                create_dir_all(parent)?;
            }
        }
        File::create(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archiver() {
        let formats = [
            ("results/test.tar", Format::Tar),
            ("results/test.zip", Format::Zip),
        ];

        for (path, expected_format) in formats.iter() {
            let archiver = create_archiver(&PathBuf::from(*path));
            assert!(archiver.is_ok());
            assert_eq!(archiver.unwrap().format(), *expected_format);
        }

        let unknown_format = create_archiver(&PathBuf::from("results/test.unknown"));
        assert!(unknown_format.is_err());
        if let Err(ToteError::UnknownFormat(msg)) = unknown_format {
            assert_eq!(msg, "test.unknown: unknown format".to_string());
        } else {
            assert!(false);
        }
    }
}

pub fn create_archiver(dest: &PathBuf) -> Result<Box<dyn Archiver>, ToteError> {
    let format = find_format(dest.file_name());
    match format {
        Ok(fmt) => Ok(match fmt {
            Format::Zip => Box::new(ZipArchiver {}),
            Format::Tar => Box::new(TarArchiver {}),
            _ => return Err(ToteError::UnknownFormat(fmt.to_string())),
        }),
        Err(e) => Err(e),
    }
}

pub fn archiver_info(archiver: &dyn Archiver, opts: &ArchiverOpts) -> String {
    format!(
        "Format: {:?}\nDestination: {:?}\nTargets: {:?}",
        archiver.format(),
        opts.dest_path(),
        opts.targets()
            .iter()
            .map(|item| item.to_str().unwrap())
            .collect::<Vec<_>>()
            .join(", ")
    )
}
