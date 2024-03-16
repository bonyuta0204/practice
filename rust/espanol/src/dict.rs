use std::collections::HashMap;
use std::fs::{self, File};
use std::io::BufReader;
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Dict {
    body: HashMap<String, String>,
}

impl Dict {
    pub fn new() -> Self {
        Self {
            body: HashMap::new(),
        }
    }

    pub fn add_entry(&mut self, word: String, translation: String) {
        self.body.insert(word, translation);
    }

    pub fn get_translation(&self, word: String) -> Option<String> {
        self.body.get(&word).map(|translation| translation.to_string())
    }

    pub fn load_from_file<P>(path: P) -> Result<Self, Box<bincode::ErrorKind>>
    where
        P: AsRef<Path>,
    {
        let db_file = File::open(&path)?;
        let mut db_data: Vec<u8> = Vec::new();

        let mut buf_reader = BufReader::new(db_file);

        buf_reader.read_to_end(&mut db_data)?;

        let dictionary: Self = bincode::deserialize(&db_data)?;

        Ok(dictionary)
    }

    pub fn save_to_file<P>(&self, path: P) -> Result<(), io::Error>
    where
        P: AsRef<Path>,
    {
        let encoded = bincode::serialize(self).unwrap();
        let dir = path.as_ref().parent();
        if let Some(dir) = dir {
            fs::create_dir_all(dir)?;
        }

        let db_file = File::create(&path)?;
        let mut buf_writer = BufWriter::new(db_file);

        buf_writer.write_all(&encoded)?;

        buf_writer.flush()?;

        Ok(())
    }
}
