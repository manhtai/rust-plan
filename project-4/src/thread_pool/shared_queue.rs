use super::ThreadPool;
use std::thread;

use super::Result;

pub struct SharedQueueThreadPool;


impl ThreadPool for SharedQueueThreadPool {
    fn new(threads: u32) -> Result<Self> where Self: Sized {
        Ok(SharedQueueThreadPool)
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static {
        thread::spawn(job);
    }
}
