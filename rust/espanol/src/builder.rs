use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Read},
};

use reqwest::blocking::get;
use tar::Archive;
use tempfile::tempdir;
use xz2::bufread::XzDecoder;

const SOURCE_URL: &str =
    "https://download.freedict.org/dictionaries/spa-eng/0.3.1/freedict-spa-eng-0.3.1.src.tar.xz";

pub fn run(path: Option<String>) -> Result<(), Error> {
    let path = path.unwrap_or_else(|| "~/.espanol/dictionary.db".to_string());
    println!("Creating dictionary for {}", path);

    // Download the dictionary source to a temporary directory
    let temp_dir = tempdir()?;
    let file_path = temp_dir.path().join("dictionary.tar.xz");
    let mut output = File::create(&file_path)?;
    let mut response = get(SOURCE_URL).expect("Failed to get response");
    let bytes = std::io::copy(&mut response, &mut output)?;

    println!("Read {} bytes from response", bytes);

    // Unpack the dictionary source
    let tar_xz = File::open(&file_path).expect("Failed to open file");
    let tar = XzDecoder::new(BufReader::new(tar_xz));
    let mut archive = Archive::new(tar);

    let mut entries = archive.entries()?.filter_map(|entry| entry.ok());

    let target_entry = entries.find(|entry| {
        entry
            .path()
            .is_ok_and(|p| p.to_str().is_some_and(|s| s.contains("spa-eng.tei")))
    });

    match target_entry {
        Some(mut entry) => {
            let mut row_data = String::new();

            entry.read_to_string(&mut row_data)?;

            println!("bytes: {}", row_data.len());

            Ok(())
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Could not find target entry",
        )),
    }
}
