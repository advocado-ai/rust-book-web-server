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
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        //take the tx inside the Option and drop it, closes the channel 
        drop(self.sender.take());
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

        ThreadPool{ 
            workers: workers, 
            sender: Some(tx) 
        }
    }

    pub fn execute<F>(&self, f:F)
    where 
        F: FnOnce() + Send + 'static,
        {
            //create a box with closure
            let job = Box::new(f);
            //mpsc send so receivers can receive
            self.sender.as_ref().expect("sender is None, execute called after ThreadPool was dropped/shutting down").send(job).expect("failed to send job: receiver was dropped because sender was dropped");
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
                let message = receiver.lock().expect("Option wrapping rx could not acquire mutex lock").recv();

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");

                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }

            }
        });

        Worker{id: id, handle: handler}
    } 
}

pub fn convert_file_request_to_bufreader_requet(raw_request_line:String)->String{

    let method_path_version_line= match raw_request_line.lines().next(){
        Some(first_line) => first_line.to_string().trim_end_matches("\r\n").to_string(),
        None => "couldn't parse first line of file request".to_string()
    };
    
    method_path_version_line
}



pub fn parse_request(request_str: String) -> Vec<String>{
    //break apart into http request type
    // http version
    // Host: 127.0.0.1:7878
    // Connection: close

    let split_request_str = request_str.split(' ').map(|s| s.to_string()).collect();

    split_request_str
}

#[cfg (test)]
mod tests{
    use super::*;
    use std::{convert, fs};

    #[test]
    fn test_parse_request(){
        let request_line = fs::read_to_string("/home/nginx/Documents/coding/rust-projects/rust-book-web-server/hello/tests/fixtures/happy_path.txt").expect("couldn't read happy path txt to string");

        let request_first_line_string = convert_file_request_to_bufreader_requet(request_line);

        let res = parse_request(request_first_line_string);

        //println!("\n\nRESULTS HERE: {:?}\n\n", res);
        assert_eq!(res, vec!["GET".to_string(), "/".to_string(), "HTTP/1.1".to_string()]);



    }
}