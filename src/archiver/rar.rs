use crate::cli::Result;
use crate::format::Format;
use crate::ArchiverOpts;

pub struct RarArchiver;

impl RarArchiver {
    pub fn new() -> Self {
        Self
    }
}

impl crate::Archiver for RarArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<()> {
        // RARアーカイブの処理を実装
        Ok(())
    }

    fn format(&self) -> Format {
        Format::Rar
    }
}
