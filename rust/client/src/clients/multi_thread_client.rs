use std::{
    error::Error,
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
};

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
    fn execute(&self, host: &'static str, count: usize) -> Result<(), Box<dyn Error>> {
        let mut handles = Vec::with_capacity(count);
        let client = Arc::new(Mutex::new(self.clone()));
        let c = Arc::new(Mutex::new(0));
        for _i in 0..self.thread_number {
            let mut stream: Option<TcpStream> = None;
            let c = Arc::clone(&c);
            let client = Arc::clone(&client);
            let handle = thread::spawn(move || {
                loop {
                    {
                        let c = c.lock().unwrap();
                        // return if already counted to 100
                        if *c >= count {
                            break;
                        }
                    }

                    if let None = stream {
                        stream = Some(TcpStream::connect(&host).unwrap());
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

                            let mut c = c.lock().unwrap();
                            *c = *c + 1;
                            if (!client.lock().unwrap().keep_alive) {
                                s.shutdown(std::net::Shutdown::Both).unwrap();
                            }
                        }
                        None => {
                            break;
                        }
                    }
                    if (!client.lock().unwrap().keep_alive) {
                        stream = None;
                    }
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
