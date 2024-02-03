
use std::{
    error::Error,
    io::{BufRead, BufReader, BufWriter, Write},
    sync::{Arc, Mutex},
    thread,
    time::{SystemTime},
};

use std::net::TcpStream;

const THREAD: usize = 4;
const ITERATION: usize = 100;

fn main() -> Result<(), Box<dyn Error>> {
    let count = Arc::new(Mutex::new(0));
    let mut handles = Vec::with_capacity(THREAD);
    let start = SystemTime::now();

    let host = "localhost:7878";

    println!("started");

    for _i in 0..THREAD {
        let count = Arc::clone(&count);
        let handle = thread::spawn(move || {
            loop {
                {
                    let count = count.lock().unwrap();
                    // return if already counted to 100
                    if *count >= ITERATION {
                        break;
                    }
                }

                let stream = TcpStream::connect(host).unwrap();

                let mut writer = BufWriter::new(&stream);
                let reader = BufReader::new(&stream);

                // Send a GET request
                let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host);
                writer.write_all(request.as_bytes()).unwrap();
                writer.flush().unwrap();

                // Read the Response

                let _response: Vec<_> = reader.lines().filter_map(|line| line.ok()).collect();

                //                println!("response: {:?}", response);

                let mut count = count.lock().unwrap();
                *count = *count + 1;
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }

    // Measure the elapsed time

    let count = count.lock().unwrap();

    let duration = start.elapsed().unwrap();
    println!("finished");
    println!("Time taken for {count} request: {:?}", duration);

    Ok(())
}
