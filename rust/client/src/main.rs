use std::{error::Error, time::Instant};

use client::clients::multi_thread_client::MultiThreadClient;
use client::clients::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let mut clients: Vec<Box<dyn Client>> = Vec::new();

    clients.push(Box::new(MultiThreadClient::new(5, true)));
    clients.push(Box::new(MultiThreadClient::new(10, true)));
    clients.push(Box::new(MultiThreadClient::new(50, true)));
    clients.push(Box::new(MultiThreadClient::new(100, true)));

    run_benchmark(clients, "abehiroshi.la.coocan.jp:80", 100);

    Ok(())
}

fn run_benchmark(clients: Vec<Box<dyn Client>>, host: &'static str, count: usize) {
    // Print header
    println!(
        "{:<20} | {:<60} | {:<10} | {:?}",
        "Client Name", "option", "Result", "Duration"
    );
    println!("{:-<1$}", "", 120); // Print a dividing line

    for client in clients {
        let start = Instant::now();

        match client.execute(host, count) {
            Ok(_) => {
                let duration = start.elapsed();
                println!(
                    "{:<20} | {:<60} | {:<10} | {:?}",
                    client.name(),
                    client.option(),
                    "Success",
                    duration
                );
            }
            Err(_e) => {
                let duration = start.elapsed();
                println!("{:<20} | {:<10} | {:?}", client.name(), "Failed", duration);
            }
        }
    }
}
