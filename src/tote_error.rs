// src/tote_error.rs
use std::fmt;
use std::io;
use zip::result::ZipError;
use walkdir::Error as WalkDirError;

#[derive(Debug)]
pub enum ToteError {
    IoError(io::Error),
    ZipError(ZipError),
    WalkDirError(WalkDirError),
    UnknownFormat(String),
}

impl fmt::Display for ToteError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ToteError::IoError(err) => write!(f, "IO error: {}", err),
            ToteError::ZipError(err) => write!(f, "ZIP error: {}", err),
            ToteError::WalkDirError(err) => write!(f, "WalkDir error: {}", err),
            ToteError::UnknownFormat(err) => write!(f, "Unknown format: {}", err),
        }
    }
}

impl From<io::Error> for ToteError {
    fn from(err: io::Error) -> ToteError {
        ToteError::IoError(err)
    }
}

impl From<ZipError> for ToteError {
    fn from(err: ZipError) -> ToteError {
        ToteError::ZipError(err)
    }
}

impl From<WalkDirError> for ToteError {
    fn from(err: WalkDirError) -> ToteError {
        ToteError::WalkDirError(err)
    }
}
