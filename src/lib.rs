pub mod util;

pub extern crate java_locator;

use std::env::set_var;
use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;

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

#[derive(thiserror::Error, Debug)]
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

pub fn download_java8() -> Result<File> {
    let (url, hash) = select_url().ok_or(Error::UnsupportedOs(()))?;
    let mut file = tempfile::tempfile()?;
    util::download_file(url, &mut file)?;
    file.seek(SeekFrom::Start(0))?;
    if hash != &util::hash_file_sha1(&mut file)? {
        return Err(Error::IOError(std::io::Error::new(std::io::ErrorKind::Other, "hash mismatch")));
    }
    file.seek(SeekFrom::Start(0))?;
    Ok(file)
}

pub fn prepare_java8() -> Result<()> {
    if let Ok(_) = java_locator::locate_java_home() {
        return Ok(());
    }
    if Path::new("./java/jdk8u402").is_dir() {
        set_var("JAVA_HOME", "./java/jdk8u402");
        return Ok(()); // TODO: Check valid.
    }
    let mut file = download_java8()?;
    zip::ZipArchive::new(&mut file)?.extract("./java")?;
    set_var("JAVA_HOME", "./java/jdk8u402");
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::prepare_java8;

    #[test]
    fn test() {
        prepare_java8().unwrap();
        assert!(java_locator::locate_java_home().is_ok());
    }
}
