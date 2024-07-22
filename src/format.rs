use std::fmt;
use crate::tote_error::ToteError;

pub enum Format {
    Tar,
    TarGz,
    TarBz2,
    Zip,
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Format::Tar => write!(f, "tar"),
            Format::TarGz => write!(f, "tar.gz"),
            Format::TarBz2 => write!(f, "tar.bz2"),
            Format::Zip => write!(f, "zip"),
        }
    }
}

pub fn detect_format(filename: &str) -> Result<Format, ToteError> {
    if filename.ends_with(".tar") {
        Ok(Format::Tar)
    } else if filename.ends_with(".tar.gz") || filename.ends_with(".tgz") {
        Ok(Format::TarGz)
    } else if filename.ends_with(".tar.bz2") || filename.ends_with(".tbz2") {
        Ok(Format::TarBz2)
    } else if filename.ends_with(".zip") {
        Ok(Format::Zip)
    } else {
        Err(ToteError::UnknownFormat(filename.to_string()))
    }
}
