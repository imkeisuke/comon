// src/archiver/tar.rs
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tar::{Builder, Archive};
use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::write::BzEncoder;
use flate2::write::ZlibEncoder;
use xz2::write::XzEncoder;
use crate::tote_error::ToteError;

pub fn create_tar_gz_archive(src_dir: &Path, tar_gz_path: &Path) -> Result<(), ToteError> {
    let tar_gz = File::create(tar_gz_path)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all(".", src_dir)?;
    Ok(())
}

pub fn create_tar_bz2_archive(src_dir: &Path, tar_bz2_path: &Path) -> Result<(), ToteError> {
    let tar_bz2 = File::create(tar_bz2_path)?;
    let enc = BzEncoder::new(tar_bz2, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all(".", src_dir)?;
    Ok(())
}

pub fn create_tar_xz_archive(src_dir: &Path, tar_xz_path: &Path) -> Result<(), ToteError> {
    let tar_xz = File::create(tar_xz_path)?;
    let enc = XzEncoder::new(tar_xz, Compression::default());
    let mut tar = Builder::new(enc);
    tar.append_dir_all(".", src_dir)?;
    Ok(())
}

pub fn extract_tar_archive(tar_path: &Path, dst_dir: &Path) -> Result<(), ToteError> {
    let tar = File::open(tar_path)?;
    let mut archive = Archive::new(tar);
    archive.unpack(dst_dir)?;
    Ok(())
}

pub fn extract_tar_gz_archive(tar_gz_path: &Path, dst_dir: &Path) -> Result<(), ToteError> {
    let tar_gz = File::open(tar_gz_path)?;
    let dec = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(dec);
    archive.unpack(dst_dir)?;
    Ok(())
}

pub fn extract_tar_bz2_archive(tar_bz2_path: &Path, dst_dir: &Path) -> Result<(), ToteError> {
    let tar_bz2 = File::open(tar_bz2_path)?;
    let dec = BzDecoder::new(tar_bz2);
    let mut archive = Archive::new(dec);
    archive.unpack(dst_dir)?;
    Ok(())
}

pub fn extract_tar_xz_archive(tar_xz_path: &Path, dst_dir: &Path) -> Result<(), ToteError> {
    let tar_xz = File::open(tar_xz_path)?;
    let dec = XzDecoder::new(tar_xz);
    let mut archive = Archive::new(dec);
    archive.unpack(dst_dir)?;
    Ok(())
}
