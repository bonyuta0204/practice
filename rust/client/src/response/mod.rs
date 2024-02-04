use std::{collections::HashMap, io::BufRead, str::from_utf8};

#[derive(Debug)]
pub struct Response {
    status: String,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn from_reader(reader: &mut impl BufRead) -> Self {
        let mut body = String::new();
        let mut lines = reader.lines();

        let mut status = String::new();

        let mut headers = HashMap::new();

        // Parse the status line
        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                continue;
            } else if line.starts_with("HTTP") {
                status = line.split_whitespace().nth(1).unwrap().to_string();
                break;
            }
        }

        // Parse header lines
        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                break;
            }

            let mut parts = line.splitn(2, ":");
            let key = parts.next().unwrap().trim().to_string();
            let value = parts.next().unwrap().trim().to_string();

            headers.insert(key, value);
        }

        // Parse body
        if let Some(content_length) = headers.get("Content-Length") {
            let content_length: usize = content_length.parse().unwrap();
            let mut buffer = vec![0; content_length];
            reader.read_exact(&mut buffer).unwrap();
            body = from_utf8(&buffer).unwrap().to_string();
        }

        Response {
            status: status,
            headers: headers,
            body: body,
        }
    }
}
