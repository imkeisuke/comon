use crate::cli::Result;
use crate::format::Format;
use crate::ArchiverOpts;

pub struct LhaArchiver;

impl LhaArchiver {
    pub fn new() -> Self {
        Self
    }
}

impl crate::Archiver for LhaArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<()> {
        // LHAアーカイブの処理を実装
        Ok(())
    }

    fn format(&self) -> Format {
        Format::LHA
    }
}
