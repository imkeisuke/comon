use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tar::Builder;
use bzip2::write::BzEncoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use walkdir::WalkDir;

pub struct TarArchiver {
    format: super::Format,
}

impl TarArchiver {
    pub fn new(format: super::Format) -> Self {
        TarArchiver { format }
    }
}

impl super::Archiver for TarArchiver {
    fn compress(&self, src: &Path, dest: &Path) -> Result<(), super::ToteError> {
        let file = File::create(dest)?;
        match self.format {
            super::Format::Tar => create_tar(&file, src),
            super::Format::TarGz => create_tar_gz(&file, src),
            super::Format::TarBz2 => create_tar_bz2(&file, src),
            _ => Err(super::ToteError::UnknownFormat(self.format.to_string())),
        }
    }

    fn decompress(&self, src: &Path, dest: &Path) -> Result<(), super::ToteError> {
        // TAR の展開は実装されていないため、適宜追加する必要があります
        Err(super::ToteError::UnknownFormat(self.format.to_string()))
    }
}

fn create_tar(file: &File, src: &Path) -> io::Result<()> {
    let mut tar = Builder::new(file);
    for entry in WalkDir::new(src).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(path)?;
            tar.append_path_with_name(&file, path.file_name().unwrap())?;
        }
    }
    tar.finish()?;
    Ok(())
}

fn create_tar_bz2(file: &File, src: &Path) -> io::Result<()> {
    let encoder = BzEncoder::new(file, bzip2::Compression::default());
    let mut tar = Builder::new(encoder);
    for entry in WalkDir::new(src).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(path)?;
            tar.append_path_with_name(&file, path.file_name().unwrap())?;
        }
    }
    tar.finish()?;
    Ok(())
}

fn create_tar_gz(file: &File, src: &Path) -> io::Result<()> {
    let encoder = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(encoder);
    for entry in WalkDir::new(src).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(path)?;
            tar.append_path_with_name(&file, path.file_name().unwrap())?;
        }
    }
    tar.finish()?;
    Ok(())
}
