use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use flate2::write::{GzEncoder, BzEncoder};
use flate2::Compression;
use tar::{Builder, Archive};
use crate::tote_error::ToteError;

pub struct TarArchiver {
    format: crate::format::Format,
}

impl TarArchiver {
    pub fn new(format: crate::format::Format) -> Self {
        TarArchiver { format }
    }

    fn create_tar<P: AsRef<Path>>(&self, src_dir: P, dest_file: P) -> Result<(), std::io::Error> {
        let tar_file = File::create(dest_file)?;
        let mut tar = Builder::new(tar_file);

        tar.append_dir_all(".", src_dir)?;

        Ok(())
    }

    fn create_tar_gz<P: AsRef<Path>>(&self, src_dir: P, dest_file: P) -> Result<(), std::io::Error> {
        let tar_gz = File::create(dest_file)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = Builder::new(enc);

        tar.append_dir_all(".", src_dir)?;

        Ok(())
    }

    fn create_tar_bz2<P: AsRef<Path>>(&self, src_dir: P, dest_file: P) -> Result<(), std::io::Error> {
        let tar_bz2 = File::create(dest_file)?;
        let enc = BzEncoder::new(tar_bz2, Compression::default());
        let mut tar = Builder::new(enc);

        tar.append_dir_all(".", src_dir)?;

        Ok(())
    }

    fn extract_tar<P: AsRef<Path>>(&self, src_file: P, dest_dir: P) -> Result<(), std::io::Error> {
        let tar_file = File::open(src_file)?;
        let mut archive = Archive::new(tar_file);

        archive.unpack(dest_dir)?;

        Ok(())
    }

    fn extract_tar_gz<P: AsRef<Path>>(&self, src_file: P, dest_dir: P) -> Result<(), std::io::Error> {
        let tar_gz = File::open(src_file)?;
        let dec = flate2::read::GzDecoder::new(tar_gz);
        let mut archive = Archive::new(dec);

        archive.unpack(dest_dir)?;

        Ok(())
    }

    fn extract_tar_bz2<P: AsRef<Path>>(&self, src_file: P, dest_dir: P) -> Result<(), std::io::Error> {
        let tar_bz2 = File::open(src_file)?;
        let dec = flate2::read::BzDecoder::new(tar_bz2);
        let mut archive = Archive::new(dec);

        archive.unpack(dest_dir)?;

        Ok(())
    }
}

impl crate::archiver::Archiver for TarArchiver {
    fn compress(&self, src: &Path, dest: &Path) -> Result<(), ToteError> {
        match self.format {
            crate::format::Format::Tar => self.create_tar(src, dest)?,
            crate::format::Format::TarGz => self.create_tar_gz(src, dest)?,
            crate::format::Format::TarBz2 => self.create_tar_bz2(src, dest)?,
            _ => return Err(ToteError::UnknownFormat(self.format.to_string())),
        }
        Ok(())
    }

    fn decompress(&self, src: &Path, dest: &Path) -> Result<(), ToteError> {
        match self.format {
            crate::format::Format::Tar => self.extract_tar(src, dest)?,
            crate::format::Format::TarGz => self.extract_tar_gz(src, dest)?,
            crate::format::Format::TarBz2 => self.extract_tar_bz2(src, dest)?,
            _ => return Err(ToteError::UnknownFormat(self.format.to_string())),
        }
        Ok(())
    }
}
