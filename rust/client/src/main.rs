use std::{
    error::Error,
    time::{Instant},
};

use client::{Client, MultiThreadClient};

fn main() -> Result<(), Box<dyn Error>> {
    let multi_thread_client = MultiThreadClient::new(4);

    run_benchmark(multi_thread_client);

    Ok(())
}

fn run_benchmark(client: impl Client) {
    let start = Instant::now();
    println!("starting benchmark for: {}", client.name());

    client.execute(String::from("localhost:7878"), 100);
    let duration = start.elapsed();

    println!(
        "Benchmark Result for: {}, result: {:?}",
        client.name(),
        duration
    );
}
