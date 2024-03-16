use std::{io::Error, path::Path};

use crate::dict::Dict;


pub fn run(word: String, path: Option<String>) -> Result<(),Error>{
    let path = match path {
        Some(path) => Path::new(&path).to_path_buf(),
        None => dirs::home_dir().unwrap().join(".espanol/directory.db"),
    };

    let dict = Dict::load_from_file(path);

    match dict {
        Ok(dict) => {
           let translation = dict.get_translation(word);
           if let Some(translation) = translation {
               println!("{}", translation)
           }
        },
        Err(_) => {
            println!("Failed to load database");
        }
    }

    Ok(())
}
