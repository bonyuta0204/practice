use std::{
    fs::File,
    io::{BufReader, Error, ErrorKind, Read},
    path::Path,
};


use quick_xml::events::Event;
use quick_xml::Reader;

use reqwest::blocking::get;
use tar::Archive;
use tempfile::tempdir;
use xz2::bufread::XzDecoder;

use crate::dict::Dict;

const SOURCE_URL: &str =
    "https://download.freedict.org/dictionaries/spa-eng/0.3.1/freedict-spa-eng-0.3.1.src.tar.xz";

pub fn run(path: Option<String>) -> Result<(), Error> {
    let path = match path {
        Some(path) => Path::new(&path).to_path_buf(),
        None => dirs::home_dir().unwrap().join(".espanol/directory.db"),
    };

    let dict_data = fetch_spanish_dict()?;
    let dictionary = parse_dict_data(dict_data);

    match dictionary {
        Ok(dictionary) => {
            dictionary.save_to_file(&path)?;
        }
        Err(_) => return Err(Error::new(ErrorKind::Other, "failed to parse XML")),
    }

    println!("created dictionary");

    Ok(())
}

fn fetch_spanish_dict() -> Result<String, Error> {
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

            Ok(row_data)
        }
        None => Err(Error::new(
            ErrorKind::NotFound,
            "Could not find target entry",
        )),
    }
}

fn parse_dict_data(row_data: String) -> Result<Dict, quick_xml::Error> {
    let mut dictonary = Dict::new();

    let buf_reader = BufReader::new(row_data.as_bytes());
    let mut reader = Reader::from_reader(buf_reader);

    let mut buf = Vec::new();
    let mut inside_orth = false;
    let mut inside_quote = false;

    let mut next_word: Option<String> = None;
    let mut next_translation: Option<String> = None;

    // Iterate through XML elements
    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => match e.local_name().as_ref() {
                b"orth" => inside_orth = true,
                b"quote" => inside_quote = true,
                b"entry" => {
                    next_word = None;
                    next_translation = None;
                }
                _ => (),
            },
            Ok(Event::End(ref e)) => match e.local_name().as_ref() {
                b"orth" => inside_orth = false,
                b"quote" => inside_quote = false,
                b"entry" => {
                    if let Some(ref word) = next_word {
                        if let Some(ref translation) = next_translation {
                            dictonary.add_entry(word.to_string(), translation.to_string())
                        }
                    }
                }
                _ => (),
            },
            Ok(Event::Text(e)) => {
                if inside_orth {
                    let word = e.unescape()?;
                    next_word = Some(word.to_string());
                } else if inside_quote {
                    let translation = e.unescape()?;
                    next_translation = Some(translation.to_string());
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e),
            _ => (),
        }
    }

    Ok(dictonary)
}
