use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use walkdir::WalkDir;
use crate::tote_error::ToteError;

pub struct ZipArchiver;

impl ZipArchiver {
    pub fn new() -> Self {
        ZipArchiver
    }
}

impl crate::archiver::Archiver for ZipArchiver {
    fn compress(&self, src: &Path, dest: &Path) -> Result<(), ToteError> {
        let file = File::create(dest)?;
        let mut zip = zip::ZipWriter::new(file);
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

        for entry in WalkDir::new(src) {
            let entry = entry?;
            let path = entry.path();
            let name = path.strip_prefix(Path::new(src))?;

            if path.is_file() {
                zip.start_file(name.to_string_lossy(), options)?;
                let mut f = File::open(path)?;
                io::copy(&mut f, &mut zip)?;
            } else if name.as_os_str().len() != 0 {
                zip.add_directory(name.to_string_lossy(), options)?;
            }
        }
        zip.finish()?;
        Ok(())
    }

    fn decompress(&self, src: &Path, dest: &Path) -> Result<(), ToteError> {
        let file = File::open(src)?;
        let mut archive = zip::ZipArchive::new(file)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let outpath = match file.enclosed_name() {
                Some(path) => PathBuf::from(dest).join(path),
                None => continue,
            };

            if (*file.name()).ends_with('/') {
                std::fs::create_dir_all(&outpath)?;
            } else {
                if let Some(p) = outpath.parent() {
                    if !p.exists() {
                        std::fs::create_dir_all(&p)?;
                    }
                }
                let mut outfile = File::create(&outpath)?;
                io::copy(&mut file, &mut outfile)?;
            }

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = file.unix_mode() {
                    std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
                }
            }
        }
        Ok(())
    }
}
