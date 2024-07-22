use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use zip::ZipWriter;
use crate::archiver::{Archiver, ArchiverOpts};
use crate::format::Format;
use crate::tote_error::ToteError;

pub struct ZipArchiver;

impl Archiver for ZipArchiver {
    fn perform(&self, inout: &ArchiverOpts) -> Result<(), ToteError> {
        let file = inout.destination()?;
        let mut zip = ZipWriter::new(file);

        for path in &inout.targets {
            if path.is_dir() {
                let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
                for entry in walkdir::WalkDir::new(path) {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_dir() {
                        zip.add_directory(path.to_string_lossy(), options)?;
                    } else {
                        zip.start_file(path.to_string_lossy(), options)?;
                        let mut f = File::open(path)?;
                        io::copy(&mut f, &mut zip)?;
                    }
                }
            } else {
                let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
                zip.start_file(path.to_string_lossy(), options)?;
                let mut f = File::open(path)?;
                io::copy(&mut f, &mut zip)?;
            }
        }

        zip.finish()?;
        Ok(())
    }

    fn format(&self) -> Format {
        Format::Zip
    }
}
