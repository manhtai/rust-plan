use super::ThreadPool;
use std::thread;

use super::Result;

pub struct NaiveThreadPool;


impl ThreadPool for NaiveThreadPool {
    fn new(threads: u32) -> Result<Self> where Self: Sized {
        Ok(NaiveThreadPool)
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static {
        thread::spawn(job);
    }
}