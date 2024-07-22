use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use tar::{Builder, Archive};
use bzip2::write::BzEncoder;
use flate2::Compression;

// TAR アーカイブを作成
pub fn create_tar(file: &File, src: &Path) -> io::Result<()> {
    let mut tar = Builder::new(file);
    // ソースディレクトリを追加
    for entry in walkdir::WalkDir::new(src).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(path)?;
            tar.append_path_with_name(&file, path.file_name().unwrap())?;
        }
    }
    tar.finish()?;
    Ok(())
}

// TAR.BZ2 アーカイブを作成
pub fn create_tar_bz2(file: &File, src: &Path) -> io::Result<()> {
    let encoder = BzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(encoder);
    // ソースディレクトリを追加
    for entry in walkdir::WalkDir::new(src).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(path)?;
            tar.append_path_with_name(&file, path.file_name().unwrap())?;
        }
    }
    tar.finish()?;
    Ok(())
}

// TAR.GZ アーカイブを作成
pub fn create_tar_gz(file: &File, src: &Path) -> io::Result<()> {
    let encoder = flate2::write::GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(encoder);
    // ソースディレクトリを追加
    for entry in walkdir::WalkDir::new(src).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if path.is_file() {
            let mut file = File::open(path)?;
            tar.append_path_with_name(&file, path.file_name().unwrap())?;
        }
    }
    tar.finish()?;
    Ok(())
}
