use std::{collections::HashMap, io::BufRead};

#[derive(Debug)]
pub struct Response {
    headers: HashMap<String, String>,
    body: Vec<String>,
}

impl Response {
    pub fn from_reader(reader: &mut impl BufRead) -> Self {
        let mut body = Vec::new();
        for line in reader.lines().filter_map(|line| line.ok()) {
            body.push(line)
        }

        Response {
            headers: HashMap::new(),
            body: body,
        }
    }
}
