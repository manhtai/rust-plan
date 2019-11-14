use super::ThreadPool;
use std::thread;

use super::Result;

pub struct RayonThreadPool;


impl ThreadPool for RayonThreadPool {
    fn new(threads: u32) -> Result<Self> where Self: Sized {
        Ok(RayonThreadPool)
    }

    fn spawn<F>(&self, job: F) where F: FnOnce() + Send + 'static {
        thread::spawn(job);
    }
}