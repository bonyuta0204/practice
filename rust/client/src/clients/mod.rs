use std::error::Error;

pub mod multi_thread_client;

pub trait Client {
    fn execute(&self, host: &'static str, count: usize) -> Result<(), Box<dyn Error>>;
    fn name(&self) -> String;
}
