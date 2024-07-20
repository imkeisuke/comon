// format.rs
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum Format {
    Zip,
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    TarZstd,
    LHA,
    Rar,
    SevenZ,
    Unknown(String),
}

pub fn find_format(file_name: Option<&std::ffi::OsStr>) -> Result<Format, ToteError> {
    if let Some(ext) = file_name.and_then(|s| s.to_str()).map(|s| s.split('.').last()) {
        match ext.to_lowercase().as_str() {
            "zip" => Ok(Format::Zip),
            "tar" => Ok(Format::Tar),
            "gz" => Ok(Format::TarGz),
            "bz2" => Ok(Format::TarBz2),
            "xz" => Ok(Format::TarXz),
            "zst" => Ok(Format::TarZstd),
            "lha" => Ok(Format::LHA),
            "rar" => Ok(Format::Rar),
            "7z" => Ok(Format::SevenZ),
            _ => Ok(Format::Unknown(ext.to_string())),
        }
    } else {
        Err(ToteError::UnknownFormat("No extension found".to_string()))
    }
}
