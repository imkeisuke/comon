use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use zip::{ZipWriter, write::FileOptions};

pub fn create_zip(file: &File, src: &Path) -> io::Result<()> {
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored).unix_permissions(0o755);
    let mut zip = ZipWriter::new(file);

    for entry in walkdir::WalkDir::new(src).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let name = path.strip_prefix(src)?;
            zip.start_file(name.to_string_lossy(), options)?;
            let mut file = File::open(path)?;
            io::copy(&mut file, &mut zip)?;
        }
    }

    zip.finish()?;
    Ok(())
}

pub fn extract_zip(file: &File, dest: &Path) -> io::Result<()> {
    let mut archive = ZipArchive::new(file)?;
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i)?;
        let file_path = dest.join(zip_file.sanitized_name());
        if zip_file.name().ends_with('/') {
            std::fs::create_dir_all(&file_path)?;
        } else {
            if let Some(parent) = file_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)?;
                }
            }
            let mut output_file = File::create(&file_path)?;
            io::copy(&mut zip_file, &mut output_file)?;
        }
    }
    Ok(())
}
