// src/archiver/zip.rs
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use zip::write::FileOptions;
use crate::tote_error::ToteError;

pub fn create_zip_archive(src_dir: &Path, zip_path: &Path) -> Result<(), ToteError> {
    let zip_file = File::create(zip_path)?;
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    for entry in WalkDir::new(src_dir
