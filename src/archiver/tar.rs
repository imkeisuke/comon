use crate::cli::Result;
use crate::format::Format;
use crate::ArchiverOpts;

pub struct TarArchiver;

impl TarArchiver {
    pub fn new() -> Self {
        Self
    }
}

impl crate::Archiver for TarArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<()> {
        // TARアーカイブの処理を実装
        Ok(())
    }

    fn format(&self) -> Format {
        Format::Tar
    }
}

// 他のTAR形式のアーカイバも同様に実装します。
