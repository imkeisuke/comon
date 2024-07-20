use crate::cli::Result;
use crate::format::Format;
use crate::ArchiverOpts;

pub struct LhaExtractor;

impl LhaExtractor {
    pub fn new() -> Self {
        Self
    }
}

impl crate::Archiver for LhaExtractor {
    fn perform(&self, inout: &ArchiverOpts) -> Result<()> {
        // LHAファイルの抽出処理を実装
        Ok(())
    }

    fn format(&self) -> Format {
        Format::LHA
    }
}
