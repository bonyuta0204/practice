use std::{error::Error, time::Instant};

use client::clients::multi_thread_client::MultiThreadClient;
use client::clients::Client;

fn main() -> Result<(), Box<dyn Error>> {
    let mut clients: Vec<Box<dyn Client>> = Vec::new();
    let multi_thread_client1 = MultiThreadClient::new(1, false);
    let multi_thread_client2 = MultiThreadClient::new(1, true);

    clients.push(Box::new(multi_thread_client1));
    clients.push(Box::new(multi_thread_client2));

    run_benchmark(clients, "abehiroshi.la.coocan.jp:80", 100);

    Ok(())
}

fn run_benchmark(clients: Vec<Box<dyn Client>>, host: &'static str, count: usize) {
    // Print header
    println!(
        "{:<20} | {:<10} | {:?}",
        "Client Name", "Result", "Duration"
    );
    println!("{:-<1$}", "", 60); // Print a dividing line

    for client in clients {
        let start = Instant::now();

        match client.execute(host, count) {
            Ok(_) => {
                let duration = start.elapsed();
                println!("{:<20} | {:<10} | {:?}", client.name(), "Success", duration);
            }
            Err(e) => {
                let duration = start.elapsed();
                println!("{:<20} | {:<10} | {:?}", client.name(), "Failed", duration);
            }
        }
    }
}
