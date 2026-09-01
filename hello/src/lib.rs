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
    NumThreadsIsZero,

}

pub enum WorkerError{
    IdNumberInvalid,

}
  
pub struct ThreadPool{
    //threads: Vec<thread::JoinHandle<()>>,
    workers: Vec<Worker>,
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

        let mut workers = Vec::with_capacity(size);

        for x in 0..size{
            //create some threads and store them in the vector
            let worker = Worker::new(x as u32);

            workers.push(worker);
        }

        ThreadPool{
            workers: workers,
        }
    }

    /* 
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError>{
        match size{
            0 => Err(PoolCreationError::NumThreadsIsZero),
            _ => {
                let mut threads = Vec::with_capacity(size);

                
                Ok(ThreadPool{threads})
            },

        }
    }
    */
    pub fn execute<F>(&self, f:F)
    where 
        F: FnOnce() + Send + 'static,
        {

        }
}

struct Worker{
    id: u32,
    handle: thread::JoinHandle<()>,

}
///Define a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure.
impl Worker{

    fn new(id: u32) -> Worker{
        let handler = thread::spawn(||{//thread code
        }
        );

        Worker{id: id, handle: handler}
    } 
}