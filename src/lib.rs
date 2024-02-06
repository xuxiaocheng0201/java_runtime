use std::env::set_var;
use std::fs::File;
use std::io::{BufReader, BufWriter, copy, Seek, SeekFrom};
use std::path::Path;
use sha1::{Digest, Sha1};
use tempfile::tempfile;
use thiserror::Error;

#[cfg(not(windows))]
compile_error!("This crate is only supported on Windows now.");

/// From https://bell-sw.com/pages/downloads/#jdk-8-lts
const fn select_url() -> Option<(&'static str, &'static str)> {
    if cfg!(target_os = "windows") {
        if cfg!(target_arch = "x86_64") {
            Some(("https://download.bell-sw.com/java/8u402+7/bellsoft-jdk8u402+7-windows-amd64.zip", "ac4f1ab7522768949f8ff5b324a4c920e8ca026f"))
        } else if cfg!(target_arch = "x86") {
            Some(("https://download.bell-sw.com/java/8u402+7/bellsoft-jdk8u402+7-windows-i586.zip", "1ccdade224c66ad9992d92b111373004b893f6cb"))
        } else {
            None
        }
    // // TODO: Check available.
    // } else if cfg!(target_os = "macos") {
    //     if cfg!(target_arch = "aarch64") {
    //         Some("https://download.bell-sw.com/java/8u402+7/bellsoft-jdk8u402+7-macos-aarch64.zip")
    //     } else if cfg!(target_arch = "arm64") {
    //         Some("https://download.bell-sw.com/java/8u402+7/bellsoft-jdk8u402+7-macos-amd64.zip")
    //     } else {
    //         None
    //     }
    // } else if cfg!(target_os = "linux") {
    //     if cfg!(target_arch = "amd64") {
    //         Some("https://download.bell-sw.com/java/8u402+7/bellsoft-jdk8u402+7-linux-amd64.tar.gz")
    //     } else if cfg!(target_arch = "aarch64") {
    //         Some("https://download.bell-sw.com/java/8u402+7/bellsoft-jdk8u402+7-linux-aarch64.tar.gz")
    //     } else if cfg!(target_arch = "ppc64le") {
    //         Some("https://download.bell-sw.com/java/8u402+7/bellsoft-jdk8u402+7-linux-ppc64le.tar.gz")
    //     } else if cfg!(target_arch = "i586") {
    //         Some("https://download.bell-sw.com/java/8u402+7/bellsoft-jdk8u402+7-linux-i586.tar.gz")
    //     } else {
    //         None
    //     } // TODO: Alpine Linux
    // // TODO: Solaris
    } else {
        None
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unsupported OS.")]
    UnsupportedOs(()),
    #[error("Network error: {0:?}")]
    NetworkError(#[from] ureq::Error),
    #[error("IO error: {0:?}")]
    IOError(#[from] std::io::Error),
    #[error("Unzip error: {0:?}")]
    UnzipError(#[from] zip::result::ZipError),
}

fn download_file(url: &str, file: &mut File) -> Result<()> {
    let mut reader = ureq::get(url).call()?.into_reader();
    let mut writer = BufWriter::new(file);
    copy(&mut reader, &mut writer)?;
    Ok(())
}

fn hash_file(file: &mut File) -> std::io::Result<String> {
    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    copy(&mut reader, &mut hasher)?;
    let hash = hasher.finalize();
    let hash = hash.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
    println!("{}", hash);
    Ok(hash)
}

// Install options: https://docs.oracle.com/javase/8/docs/technotes/guides/install/config.html#table_config_file_options
pub fn prepare_java8() -> Result<()> {
    if let Ok(_) = java_locator::locate_java_home() {
        return Ok(());
    }
    if Path::new("./java").is_dir() {
        return Ok(()); // TODO: Check valid.
    }
    let (url, hash) = select_url().ok_or(Error::UnsupportedOs(()))?;
    println!("{}", hash);
    let mut file = tempfile()?;
    download_file(url, &mut file)?;
    file.seek(SeekFrom::Start(0))?;
    if hash != &hash_file(&mut file)? {
        return Err(Error::IOError(std::io::Error::new(std::io::ErrorKind::Other, "Hash mismatch")));
    }
    file.seek(SeekFrom::Start(0))?;
    zip::ZipArchive::new(&mut file)?.extract("./")?;
    set_var("JAVA_HOME", "./jdk8u402");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::prepare_java8;

    #[test]
    fn test() {
        prepare_java8().unwrap();
    }
}