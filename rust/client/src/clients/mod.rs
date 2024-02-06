use std::error::Error;

pub mod hyper_client;
pub mod multi_thread_client;
pub mod pipeline_client;

pub trait Client {
    fn execute(&self, host: &'static str, count: usize) -> Result<(), Box<dyn Error>>;
    fn option(&self) -> String;
    fn name(&self) -> String;
}
