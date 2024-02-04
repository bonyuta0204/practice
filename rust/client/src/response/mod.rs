use std::{collections::HashMap, io::BufRead, str::from_utf8};

#[derive(Debug)]
pub struct Response {
    protocol: String,
    status: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn from_reader(reader: &mut impl BufRead) -> Self {
        let mut body = Vec::new();
        let mut protocol = String::new();
        let mut lines = reader.lines();

        let mut status = String::new();

        let mut headers = HashMap::new();

        // Parse the status line
        while let Some(Ok(line)) = lines.next() {
            if line.is_empty() {
                continue;
            } else if line.starts_with("HTTP") {
                let mut parts = line.split_whitespace();

                protocol = parts.next().unwrap().to_string();
                status = parts.next().unwrap().to_string();

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
            body = buffer
        }

        Response {
            protocol: protocol,
            status: status,
            headers: headers,
            body: body,
        }
    }

    pub fn print(&self) {
        println!("Protocol: {}", self.protocol);
        println!("Status: {}", self.status);
        println!("Headers");
        for (key, value) in &self.headers {
            println!("{}: {}", key, value);
        }
        println!("Body: {}", String::from_utf8_lossy(&self.body));
    }
}
