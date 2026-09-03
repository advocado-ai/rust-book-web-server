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

use std::{
    sync::{Arc, Mutex,mpsc},
    thread,
};

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
    sender: mpsc::Sender<Job>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in self.workers.drain(..){
            println!("Shutting down worker {}", worker.id);

            worker.handle.join().expect(" thread on drop");
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

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

        let (tx, rx) = mpsc::channel();

        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            //create some threads and store them in the vector
            let worker = Worker::new(id, Arc::clone(&rx));

            workers.push(worker);
        }

        ThreadPool{ workers: workers, sender: tx }
    }

    pub fn execute<F>(&self, f:F)
    where 
        F: FnOnce() + Send + 'static,
        {
            //create a box with closure
            let job = Box::new(f);
            //mpsc send so receivers can receive
            self.sender.send(job).expect("failed to send job");
        }
}

struct Worker{
    id: usize,
    //receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    handle: thread::JoinHandle<()>,

}
///Define a Worker::new function that takes an id number and returns a Worker instance that holds the id and a thread spawned with an empty closure.
impl Worker{
    ///receives an Arc::clone of rx receiver
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        let handler = thread::spawn(move ||{//thread code
            //use loop not while let as while let will not drop rhs values so holds on to the mutex lock for duration of job so other workers cannot receive jobs. 
            loop {
                //recv blocks so if there is no job yet, current thread will wait until a job becomes available
             
                let job = receiver.lock().expect("rx in worker's closure couldn't acquire mutex lock").recv().expect("couldn't receive job inside Arc::mutex(rx)");

                println!("Worker {id} got a job; executing.");

                job();
            }
        });

        Worker{id: id, handle: handler}
    } 
}