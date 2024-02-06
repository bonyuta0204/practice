#[derive(Debug)]
pub struct Request {
    method: String,
    path: String,
    protocol: String,
    host: String,
}

impl Request {
    pub fn new(method: String, path: String, protocol: String, host: String) -> Self {
        Self {
            method,
            path,
            protocol,
            host,
        }
    }

    pub fn get(host: &str, path: &str) -> Self {
        Self {
            method: "GET".to_string(),
            path: path.to_string(),
            protocol: "HTTP/1.1".to_string(),
            host: host.to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}\r\nHost: {}",
            self.method, self.path, self.protocol, self.host
        )
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.to_string().as_bytes().to_vec()
    }
}
