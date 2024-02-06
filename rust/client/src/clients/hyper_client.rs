use super::Client;

use futures::future::join_all;
use http_body_util::Empty;
use hyper::body::Bytes;
use hyper::Request;
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;

pub struct HyperClient {
    // number of threads for processing
    runtime: Runtime,
}

impl HyperClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        Ok(Self { runtime })
    }
}

impl Client for HyperClient {
    fn name(&self) -> String {
        String::from("HyperClient")
    }
    fn option(&self) -> String {
        format!("")
    }
    fn execute(&self, host: &'static str, count: usize) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.runtime.block_on(async {
            let stream = TcpStream::connect(host).await.expect("failed to connect");
            let io = TokioIo::new(stream);

            let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await.unwrap();

            // Spawn a task to poll the connection, driving the HTTP state
            tokio::task::spawn(async move {
                if let Err(err) = conn.await {
                    println!("Connection failed: {:?}", err);
                }
            });

            // Create an HTTP request with an empty body and a HOST header
            let req = Request::builder()
                .uri(format!("http://{}", host))
                .header(hyper::header::HOST, host)
                .body(Empty::<Bytes>::new())
                .unwrap();

            println!("request: {:?}", req);

            let send_requests = (0..count).map(|_| sender.send_request(req.clone()));

            let responses = join_all(send_requests).await;

            for res in responses {
                match res {
                    Ok(res) => {
                        println!("response: {:?}", res);
                    }
                    Err(e) => {
                        println!("error: {:?}", e);
                    }
                }
            }
        });
        Ok(())
    }
}
