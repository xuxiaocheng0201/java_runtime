use std::fs::File;
use std::io::{BufReader, BufWriter, copy};
use sha1::{Digest, Sha1};

pub fn download_file(url: &str, file: &mut File) -> crate::Result<()> {
    let mut reader = ureq::get(url).call()?.into_reader();
    let mut writer = BufWriter::new(file);
    copy(&mut reader, &mut writer)?;
    Ok(())
}

pub fn hash_file_sha1(file: &mut File) -> std::io::Result<String> {
    let mut reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    copy(&mut reader, &mut hasher)?;
    let hash = hasher.finalize();
    let hash = hash.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join("");
    println!("{}", hash);
    Ok(hash)
}
