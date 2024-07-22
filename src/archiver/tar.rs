use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use flate2::Compression;
use flate2::write::GzEncoder;
use tar::{Builder, Archive};

use crate::archiver::{Archiver, ArchiverOpts};
use crate::format::Format;
use crate::tote_error::ToteError;

pub struct TarArchiver;

impl Archiver for TarArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<(), ToteError> {
        let file = inout.destination()?;
        let tar_gz = GzEncoder::new(file, Compression::default());
        let mut tar = Builder::new(tar_gz);

        for path in &inout.targets {
            if path.is_dir() {
                tar.append_dir_all(path.file_name().unwrap(), path)?;
            } else {
                tar.append_path(path)?;
            }
        }

        tar.finish()?;
        Ok(())
    }

    fn format(&self) -> Format {
        Format::Tar
    }
}

pub struct TarGzArchiver;

impl Archiver for TarGzArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<(), ToteError> {
        let file = inout.destination()?;
        let tar_gz = GzEncoder::new(file, Compression::default());
        let mut tar = Builder::new(tar_gz);

        for path in &inout.targets {
            if path.is_dir() {
                tar.append_dir_all(path.file_name().unwrap(), path)?;
            } else {
                tar.append_path(path)?;
            }
        }

        tar.finish()?;
        Ok(())
    }

    fn format(&self) -> Format {
        Format::TarGz
    }
}

pub struct TarBz2Archiver;

impl Archiver for TarBz2Archiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<(), ToteError> {
        let file = inout.destination()?;
        let tar_bz2 = flate2::write::BzEncoder::new(file, Compression::default());
        let mut tar = Builder::new(tar_bz2);

        for path in &inout.targets {
            if path.is_dir() {
                tar.append_dir_all(path.file_name().unwrap(), path)?;
            } else {
                tar.append_path(path)?;
            }
        }

        tar.finish()?;
        Ok(())
    }

    fn format(&self) -> Format {
        Format::TarBz2
    }
}

pub struct TarXzArchiver;

impl Archiver for TarXzArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<(), ToteError> {
        let file = inout.destination()?;
        let tar_xz = xz2::write::XzEncoder::new(file, 9);
        let mut tar = Builder::new(tar_xz);

        for path in &inout.targets {
            if path.is_dir() {
                tar.append_dir_all(path.file_name().unwrap(), path)?;
            } else {
                tar.append_path(path)?;
            }
        }

        tar.finish()?;
        Ok(())
    }

    fn format(&self) -> Format {
        Format::TarXz
    }
}

pub struct TarZstdArchiver;

impl Archiver for TarZstdArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<(), ToteError> {
        let file = inout.destination()?;
        let tar_zstd = zstd::stream::write::Encoder::new(file, 0)?;
        let mut tar = Builder::new(tar_zstd);

        for path in &inout.targets {
            if path.is_dir() {
                tar.append_dir_all(path.file_name().unwrap(), path)?;
            } else {
                tar.append_path(path)?;
            }
        }

        tar.finish()?;
        Ok(())
    }

    fn format(&self) -> Format {
        Format::TarZstd
    }
}
