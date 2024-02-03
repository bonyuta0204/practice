use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PrefixComponent,
    thread,
};

use hello::ThreadPool;
fn main() {
    let listener = match TcpListener::bind("127.0.0.1:7878") {
        Ok(listenr) => listenr,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            return;
        }
    };

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = match stream {
            Ok(stream) => stream,
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
                continue;
            }
        };

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .filter_map(|result| result.ok())
        .take_while(|line| !line.is_empty())
        .collect();

    if let Some(request_line) = http_request.first() {
        let (status_line, file_name) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "hello.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
        match fs::read_to_string(file_name) {
            Ok(contents) => {
                let length = contents.len();
                let response =
                    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("cannot read from file. {}", e);
            }
        };
    } else {
        eprint!("malformated HTTP Request")
    };
}
