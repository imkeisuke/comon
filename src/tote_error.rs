use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ToteError {
    IO(io::Error),
    UnknownFormat(String),
    FileExists(String),
}

impl fmt::Display for ToteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToteError::IO(e) => write!(f, "IO error: {}", e),
            ToteError::UnknownFormat(msg) => write!(f, "Unknown format: {}", msg),
            ToteError::FileExists(msg) => write!(f, "File exists: {}", msg),
        }
    }
}

impl From<io::Error> for ToteError {
    fn from(err: io::Error) -> ToteError {
        ToteError::IO(err)
    }
}

impl std::error::Error for ToteError {}
