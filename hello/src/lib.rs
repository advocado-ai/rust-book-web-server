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
    collections::{HashMap},
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

/*
GET / HTTP/1.1          <- request line: method, path, version
Host: 127.0.0.1:7878    <- header
Connection: close       <- header
                         <- blank line (separator, marks end of headers)
[optional body here]    <- body (absent in a GET like this)
*/

//collects request line
pub fn collect_request_line(raw_request_line:String)->String{
    //returns first line of method path version 
    let method_path_version_line= match raw_request_line.lines().next(){
        Some(first_line) => first_line.to_string().trim_end_matches("\r\n").to_string(),
        None => "couldn't parse request line (first line) of request".to_string()
    };
    
    method_path_version_line
}

//parse request line into parts
pub fn parse_request_line(request_str: String) -> Vec<String>{
    //break apart into [method, path, version]
    //ex. GET / HTTP/1.1
    let split_request_str = request_str.split(' ').map(|s| s.to_string()).collect();

    split_request_str
}

pub fn collect_headers(raw_request_line:String)-> HashMap<String, String>{
    //collect headers using take_while line is not empty, collect all until blank line 

    //throw away first line
    let _ = raw_request_line.lines().next();

    //collect the rest into a hashmap
    let mut header_lines = Vec::new();

    //each line gets helper called to parse string into hashmap to insert into this outer parent hashmap
    header_lines = raw_request_line.lines().take_while(|line| ! line.is_empty()).collect();
    
    header_lines.into_iter().map(|line|parse_header_line(line.trim().to_string()));

    let mut headers_hashmap = HashMap::<String,String>::new();

    header_lines.into_iter().map(|k,v|headers_hashmap.insert(k,v));

    headers_hashmap

}

pub fn parse_header_line(header_line: String)-> HashMap<String, String>{
    //parse each key value pair header line string into a hashmap k:v 
    let mut split_header_line = header_line.split(':');

    let mut single_hashmap = HashMap::<String, String>::new();

    let header_key = match split_header_line.next(){
        Some(hk) => hk.to_string(),
        None =>  "no header".to_string(),
    };

    let header_value = match split_header_line.next(){
        Some(hv) => hv.to_string(),
        None => "no value".to_string(),
    };

    single_hashmap.insert(header_key, header_value);

    single_hashmap


}




//////////////////////////////////////////////


#[cfg (test)]
mod tests{
    use super::*;
    use std::{convert, fs};

    #[test]
    ///fn test_parse_request_line also tests fixture happy_path.txt
    fn test_parse_request_line(){
        let request_line = fs::read_to_string("/home/nginx/Documents/coding/rust-projects/rust-book-web-server/hello/tests/fixtures/happy_path.txt").expect("couldn't read happy path txt to string");

        let request_first_line_string = collect_request_line(request_line);

        let res = parse_request_line(request_first_line_string);

        //println!("\n\nRESULTS HERE: {:?}\n\n", res);
        assert_eq!(res, vec!["GET".to_string(), "/".to_string(), "HTTP/1.1".to_string()]);

    }

    #[test]
    fn test_empty_request(){
        let request_line = fs::read_to_string("tests/fixtures/empty_request_line.txt").expect("couldn't read empty request to string");

        let req_first_line = collect_request_line(request_line);

        let res = parse_request_line(req_first_line);

        println!("\n\nRESULTS HERE: {:?}\n\n", res);

        assert_eq!(res, vec![""]);
    }

    #[test]
    fn test_missing_http_version(){
        let request_line = fs::read_to_string("tests/fixtures/missing_http_version.txt").expect("couldn't read empty request to string");

        let req_first_line = collect_request_line(request_line);

        let res = parse_request_line(req_first_line);

        println!("\n\nRESULTS HERE: {:?}\n\n", res);

        assert_eq!(res, vec!["GET".to_string(), "/".to_string()]);

    }

    #[test]
    fn test_no_trailing_blank_line(){
        let request_line = fs::read_to_string("tests/fixtures/no_trailing_blank_line.txt").expect("couldn't read empty request to string");

        let req_first_line = collect_request_line(request_line);

        let res = parse_request_line(req_first_line);

        println!("\n\nRESULTS HERE: {:?}\n\n", res);

        assert_eq!(res, vec!["GET"]);
    }

}