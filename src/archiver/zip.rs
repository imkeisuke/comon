use crate::cli::Result;
use crate::format::Format;
use crate::ArchiverOpts;

pub struct ZipArchiver;

impl ZipArchiver {
    pub fn new() -> Self {
        Self
    }
}

impl crate::Archiver for ZipArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<()> {
        // ZIPアーカイブの処理を実装
        Ok(())
    }

    fn format(&self) -> Format {
        Format::Zip
    }
}
