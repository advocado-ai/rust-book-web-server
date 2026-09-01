/*
impl std::fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PoolCreationError::NumThreadIsZero => write!(f, "..."),
        }
    }
}

impl std::error::Error for PoolCreationError {}


*/

use std::thread;

#[derive(Debug)]
pub enum PoolCreationError{
    NumThreadIsZero,

}

  
pub struct ThreadPool{
    threads: Vec<thread::JoinHandle<()>>,
}

impl ThreadPool{
    ///Create a new ThreadPool
        ///
        /// Size is the number of threads in the pool
        /// 
        /// # Panics
        /// 
        /// The 'new' function panics if size is zero
        
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);

        let mut threads = Vec::with_capacity(size);

        for _ in 0..size{
            //create some threads and store them in the vector
        }

        ThreadPool{
            threads
        }
    }

    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>{
        match size{
            0 => Err(PoolCreationError::NumThreadIsZero),
            _ => {
                let mut threads = Vec::with_capacity(size);
                Ok(ThreadPool{threads})
            },

        }
    }

    pub fn execute<F>(&self, f:F)
    where 
        F: FnOnce() + Send + 'static,
        {

        }
}