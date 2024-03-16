use std::{
    fs::File,
    io::{self, BufReader},
};

use reqwest::blocking::get;
use tar::Archive;
use tempfile::tempdir;
use xz2::bufread::XzDecoder;

const SOURCE_URL: &str =
    "https://download.freedict.org/dictionaries/spa-eng/0.3.1/freedict-spa-eng-0.3.1.src.tar.xz";

pub fn run(path: Option<String>) -> Result<(), io::Error> {
    let path = path.unwrap_or_else(|| "~/.espanol/dictionary.db".to_string());
    println!("Creating dictionary for {}", path);

    // Download the dictionary source to a temporary directory
    let temp_dir = tempdir()?;
    let file_path = temp_dir.path().join("dictionary.tar.xz");
    let mut output = File::create(&file_path)?;
    let mut response = get(SOURCE_URL).expect("Failed to get response");
    std::io::copy(&mut response, &mut output)?;

    // Unpack the dictionary source
    let tar_xz = File::open(&file_path).expect("Failed to open file");
    let tar = XzDecoder::new(BufReader::new(tar_xz));
    let mut archive = Archive::new(tar);
    archive.unpack(&path).expect("Failed to unpack tar file");

    println!("created db to {}", path);


    Ok(())
}
