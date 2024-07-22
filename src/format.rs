// src/format.rs
use crate::tote_error::ToteError;
use std::fmt;

#[derive(Debug)]
pub enum Format {
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    Zip,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Format::Tar => write!(f, "tar"),
            Format::TarGz => write!(f, "tar.gz"),
            Format::TarBz2 => write!(f, "tar.bz2"),
            Format::TarXz => write!(f, "tar.xz"),
            Format::Zip => write!(f, "zip"),
        }
    }
}

pub fn get_format_from_extension(ext: &str) -> Result<Format, ToteError> {
    match ext {
        "tar" => Ok(Format::Tar),
        "gz" => Ok(Format::TarGz),
        "bz2" => Ok(Format::TarBz2),
        "xz" => Ok(Format::TarXz),
        "zip" => Ok(Format::Zip),
        _ => Err(ToteError::UnknownFormat(ext.to_string())),
    }
}

pub fn get_format_from_path(path: &std::path::Path) -> Result<Format, ToteError> {
    match path.extension().and_then(|s| s.to_str()) {
        Some(ext) => get_format_from_extension(ext),
        None => Err(ToteError::UnknownFormat("No extension found".to_string())),
    }
}
