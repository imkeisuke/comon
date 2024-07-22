pub mod tar;
pub mod zip;

use crate::format::Format;
use crate::tote_error::ToteError;
use std::path::Path;

pub trait Archiver {
    fn compress(&self, src: &Path, dest: &Path) -> Result<(), ToteError>;
    fn decompress(&self, src: &Path, dest: &Path) -> Result<(), ToteError>;
}

pub fn create_archiver(format: Format) -> Result<Box<dyn Archiver>, ToteError> {
    match format {
        Format::Tar | Format::TarGz | Format::TarBz2 => Ok(Box::new(tar::TarArchiver::new(format))),
        Format::Zip => Ok(Box::new(zip::ZipArchiver::new())),
        _ => Err(ToteError::UnknownFormat(format.to_string())),
    }
}
