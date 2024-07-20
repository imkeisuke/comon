use crate::cli::Result;
use crate::format::Format;
use crate::ArchiverOpts;

pub struct SevenZExtractor;

impl SevenZExtractor {
    pub fn new() -> Self {
        Self
    }
}

impl crate::Archiver for SevenZExtractor {
    fn perform(&self, inout: &ArchiverOpts) -> Result<()> {
        // 7zファイルの抽出処理を実装
        Ok(())
    }

    fn format(&self) -> Format {
        Format::SevenZ
    }
}
