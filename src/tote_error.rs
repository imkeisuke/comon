use std::fmt;

#[derive(Debug)]
pub enum ToteError {
    IoError(std::io::Error),
    ZipError(zip::ZipError),
    FormatError(String),
    UnknownFormat(String),
}

impl fmt::Display for ToteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ToteError::IoError(err) => write!(f, "IO error: {}", err),
            ToteError::ZipError(err) => write!(f, "Zip error: {}", err),
            ToteError::FormatError(err) => write!(f, "Format error: {}", err),
            ToteError::UnknownFormat(fmt) => write!(f, "Unknown format: {}", fmt),
        }
    }
}

impl From<std::io::Error> for ToteError {
    fn from(error: std::io::Error) -> Self {
        ToteError::IoError(error)
    }
}

impl From<zip::ZipError> for ToteError {
    fn from(error: zip::ZipError) -> Self {
        ToteError::ZipError(error)
    }
}
