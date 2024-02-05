use std::{
    error::Error,
    io::{BufReader, BufWriter, Write},
    net::TcpStream,
    thread,
};

use super::Client;
use crate::response::Response;

#[derive(Clone)]
pub struct PipeLineClient {
    // number of threads for processing
    thread_number: usize,
}
impl PipeLineClient {
    pub fn new(thread_number: usize) -> Self {
        Self { thread_number }
    }
}

impl Client for PipeLineClient {
    fn name(&self) -> String {
        String::from("PipeLineClient")
    }
    fn option(&self) -> String {
        format!("thread_number: {}", self.thread_number)
    }
    fn execute(&self, host: &'static str, count: usize) -> Result<(), Box<dyn Error>> {
        let mut handles = Vec::with_capacity(count);

        // Calculate the base number of jobs per thread and the remainder
        let base_job_count = count / self.thread_number;
        let remainder = count % self.thread_number;

        // Initialize job_counts with base_job_count
        let mut job_counts = vec![base_job_count; self.thread_number];

        // Distribute the remainder among the first 'remainder' threads
        for i in 0..remainder {
            job_counts[i] += 1;
        }

        for job_count in job_counts {
            let handle = thread::spawn(move || {
                let stream: TcpStream = TcpStream::connect(&host).unwrap();
                let mut writer = BufWriter::new(&stream);
                let mut reader = BufReader::new(&stream);

                let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host);
                for _ in 0..job_count {
                    writer.write_all(request.as_bytes()).unwrap();
                }
                writer.flush().unwrap();

                for _ in 0..job_count {
                    let response = Response::from_reader(&mut reader);
                    // response.print()
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
