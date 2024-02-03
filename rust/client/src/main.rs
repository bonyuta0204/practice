use std::{error::Error, time::Instant};

use client::clients::multi_thread_client::MultiThreadClient;
use client::clients::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let mut clients: Vec<Box<dyn Client>> = Vec::new();
    let multi_thread_client = MultiThreadClient::new(4);

    clients.push(Box::new(multi_thread_client));

    run_benchmark(clients, "localhost:7878", 100);

    Ok(())
}

fn run_benchmark(clients: Vec<Box<dyn Client>>, host: &'static str, count: usize) {
    for client in clients {
        let start = Instant::now();
        println!("starting benchmark for: {}", client.name());

        match client.execute(host, count) {
            Ok(_) => {
                let duration = start.elapsed();

                println!(
                    "Benchmark Result for: {}, result: {:?}",
                    client.name(),
                    duration
                );
            }
            Err(e) => {
                let duration = start.elapsed();
                println!(
                    "Benchmark Failed for: {}, result: {:?}",
                    client.name(),
                    duration
                );
            }
        }
    }
}
