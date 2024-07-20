use crate::cli::Result;
use crate::format::Format;
use crate::ArchiverOpts;

pub struct SevenZArchiver;

impl SevenZArchiver {
    pub fn new() -> Self {
        Self
    }
}

impl crate::Archiver for SevenZArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<()> {
        // 7zアーカイブの処理を実装
        Ok(())
    }

    fn format(&self) -> Format {
        Format::SevenZ
    }
}
