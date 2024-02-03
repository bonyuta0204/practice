pub mod multi_thread_client;

pub trait Client {
    fn execute(&self, host: String, count: usize);
    fn name(&self) -> String;
}
