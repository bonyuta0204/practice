use core::time;
use std::{
    io::Write,
    sync::{Arc, Mutex},
    thread,
};

use std::net::TcpStream;

const THREAD: usize = 4;
const ITERATION: usize = 100;

fn main() {
    let count = Arc::new(Mutex::new(0));
    let mut handles = Vec::with_capacity(THREAD);

    for i in 0..THREAD {
        let count = count.clone();
        let handle = thread::spawn(move || {
            loop {
                let mut stream = TcpStream::connect("localhost:7878").unwrap();
                {
                    let count = count.lock().unwrap();
                    // return if already counted to 100
                    if *count >= ITERATION {
                        break;
                    }
                }

                stream.write_all(b"GET / HTTP/1.1\r\n").unwrap();

                let mut count = count.lock().unwrap();
                *count = *count + 1;
                println!("counted to {count} in thread: {i}");
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.join();
    }
}
