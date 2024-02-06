use std::{
    error::Error,
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
    sync::{mpsc, Arc, Mutex},
    thread,
};

use hyper::Uri;

use super::Client;
use crate::response::Response;

#[derive(Clone)]
pub struct MultiThreadClient {
    // number of threads for processing
    thread_number: usize,
    keep_alive: bool,
}
impl MultiThreadClient {
    pub fn new(thread_number: usize, keep_alive: bool) -> Self {
        Self {
            thread_number,
            keep_alive,
        }
    }
}

impl Client for MultiThreadClient {
    fn name(&self) -> String {
        String::from("MultiThreadClient")
    }
    fn option(&self) -> String {
        format!(
            "thread_number: {}, keep_alive: {}",
            self.thread_number, self.keep_alive
        )
    }
    fn execute(&self, host: &'static str, count: usize) -> Result<(), Box<dyn Error>> {
        let uri = host.parse::<Uri>().unwrap();
        let authority = uri.authority().unwrap();
        let port = match uri.port() {
            Some(p) => p.as_str().to_string(),
            None => "80".to_string(),
        };

        let addr = format!("{}:{}", authority, port);

        let mut handles = Vec::with_capacity(count);
        let client = Arc::new(Mutex::new(self.clone()));
        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        for i in 0..count {
            tx.send(i).unwrap();
        }

        for i in 0..self.thread_number {
            let mut stream: Option<TcpStream> = None;
            let client = Arc::clone(&client);
            let rx = Arc::clone(&rx);
            let addr = addr.clone();

            let handle = thread::spawn(move || loop {
                let c = rx.lock().unwrap().try_recv();

                match c {
                    Ok(c) => {
                        //    println!("Thread: {} received request: {}", i, c);
                    }
                    Err(_) => {
                        break;
                    }
                }

                if let None = stream {
                    stream = Some(TcpStream::connect(&addr).unwrap());
                }

                match &stream {
                    Some(s) => {
                        let mut writer = BufWriter::new(s);
                        let mut reader = BufReader::new(s);

                        // Send a GET request
                        let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host);
                        writer.write_all(request.as_bytes()).unwrap();
                        writer.flush().unwrap();

                        let response = Response::from_reader(&mut reader);

                        // response.print();

                        if !client.lock().unwrap().keep_alive {
                            s.shutdown(std::net::Shutdown::Both).unwrap();
                        }
                    }
                    None => {
                        break;
                    }
                }
                if !client.lock().unwrap().keep_alive {
                    stream = None;
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Err(_e) = handle.join() {
                return Err("thread panicked".into());
            }
        }

        Ok(())
    }
}
