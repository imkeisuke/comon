use crate::cli::Result;
use crate::format::Format;
use crate::ArchiverOpts;

pub struct RarExtractor;

impl RarExtractor {
    pub fn new() -> Self {
        Self
    }
}

impl crate::Archiver for RarExtractor {
    fn perform(&self, inout: &ArchiverOpts) -> Result<()> {
        // RARファイルの抽出処理を実装
        Ok(())
    }

    fn format(&self) -> Format {
        Format::Rar
    }
}
