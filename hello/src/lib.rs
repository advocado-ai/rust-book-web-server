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

pub mod response;
pub mod request;

use std::{
    collections::HashMap, option, sync::{Arc, Mutex,mpsc}, thread,
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
reference
GET / HTTP/1.1          <- request line: method, path, version
Host: 127.0.0.1:7878    <- header
Connection: close       <- header
                         <- blank line (separator, marks end of headers)
[optional body here]    <- body (absent in a GET like this)
*/

//collects request line
pub fn collect_request_line(raw_request_line:&str)->Option<String>{
    //returns first line of method path version 
    match raw_request_line.lines().next(){
        //lines already strips terminators (trim_end_matches("\r\n").)
        Some(first_line) => Some(first_line.to_string()),
        None => None,
    }
    
}

/*
pub fn parse_request_line(request_line: &str) -> Option<(String, String, String)> {
    let mut parts = request_line.split(' ');
    let (Some(method), Some(path), Some(version), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return None;
    };

    if method.is_empty() || path.is_empty() || version.is_empty() {
        return None;
    }

    Some((method.to_string(), path.to_string(), version.to_string()))
}

*/

//parse request line into parts
pub fn parse_request_line(request_line: &str) -> Option<(String, String, String)>{
    let mut parts = request_line.split(' ');
    let (Some(method), Some(path), Some(version), None) = (parts.next(), parts.next(), parts.next(), parts.next())
    else{
        return None; // 400 error
    };

    if method.is_empty() || path.is_empty() || version.is_empty(){
        return None; // 400 error
    }

    Some((method.to_string(), path.to_string(), version.to_string()))

}

pub fn collect_headers(raw_request_line:&str)-> HashMap<String, String>{
    //collect headers using take_while line is not empty, collect all until blank line 

    //each line gets helper called to parse string into hashmap to insert into this outer parent hashmap
    let mut headers_hashmap = HashMap::<String,String>::new();

    raw_request_line
        .lines()
        .skip(1)
        .take_while(|line|!line.is_empty())
        .filter_map(|line| parse_header_line(&line.to_string().trim()))
        .for_each(|(k,v)|{
            headers_hashmap.insert(k,v);
        });

    headers_hashmap

}

pub fn parse_header_line(header_line: &str)-> Option<(String, String)>{
    //parse each key value pair header line string into a tuple pair k,v
    match header_line.split_once(':'){
        Some((key_ref, value_ref)) => Some((key_ref.trim().to_string(), value_ref.trim().to_string())),
        None => None,
    }
}

/////////////////////////////////////////////


#[cfg (test)]
mod tests{
    use super::*;
    use std::{convert, fs};

    #[test]
    fn test_empty_request(){
        let request_line = fs::read_to_string("tests/fixtures/empty_request_line.txt").expect("couldn't read empty request to string");

        let req_first_line = collect_request_line(&request_line).unwrap();

        let res = parse_request_line(&req_first_line);

        println!("\n\nRESULTS HERE empty request:\n\n {:?}\n\n", res);

        assert_eq!(res, None);

        let headers_hashmap = collect_headers(&request_line);

        assert_eq!(headers_hashmap.len(), 0);

    }

    #[test]
    ///fn test_parse_request_line also tests fixture happy_path.txt
    fn test_happy_path(){
        
        //TEST REQUEST LINE
        let request_line = fs::read_to_string("/home/nginx/Documents/coding/rust-projects/rust-book-web-server/hello/tests/fixtures/happy_path.txt").expect("couldn't read happy path txt to string");

        let request_first_line_string = collect_request_line(&request_line).unwrap();

        let res = parse_request_line(&request_first_line_string);

        println!("\n\nRESULTS HERE: {:?}\n\n", res);
        assert_eq!(res, Some(("GET".to_string(), "/".to_string(), "HTTP/1.1".to_string())));

        //TEST HEADERS
        let headers_hashmap = collect_headers(&request_line);

        let mut answers_vec = vec![("Host".to_string(), "127.0.0.1:7878".to_string()), ("Connection".to_string(), "close".to_string())];

        assert_eq!(headers_hashmap.len(), 2);

        println!("\n\nRESULTS HERE FOR TEST PARSE REQUEST LINE:\n\n");

        let mut count = 0;

        for (k,v) in &headers_hashmap{
            println!("k-{}:v-{}", k, v);

        }

        assert_eq!(headers_hashmap.get("Host"), Some(&"127.0.0.1:7878".to_string()));

        assert_eq!(headers_hashmap.get("Connection"), Some(&"close".to_string()));

    }

    #[test]
    fn test_missing_http_version(){
        let request_line = fs::read_to_string("tests/fixtures/missing_http_version.txt").expect("couldn't read empty request to string");

        let req_first_line = collect_request_line(&request_line).unwrap();

        let res = parse_request_line(&req_first_line);

        println!("\n\nRESULTS HERE missing http version:\n\n {:?}\n\n", res);

        assert_eq!(res, None);

        let header_hashmap = collect_headers(&request_line);

        assert_eq!(header_hashmap.len(), 1);
        assert_eq!(header_hashmap.get("Host"), Some(&"127.0.0.1:7878".to_string()));

    }
    
    #[test]
    fn test_no_trailing_blank_line(){
        let request_line = fs::read_to_string("tests/fixtures/no_trailing_blank_line.txt").expect("couldn't read empty request to string");

        let req_first_line = collect_request_line(&request_line).unwrap();

        let res = parse_request_line(&req_first_line);

        println!("\n\nRESULTS HERE no trailing blank line:\n\n {:?}\n\n", res);

        assert_eq!(res, Some(("GET".to_string(), "/".to_string(), "HTTP/1.1".to_string())));

        let header_hashmap = collect_headers(&request_line);

        assert_eq!(header_hashmap.len(), 1);

        assert_eq!(header_hashmap.get("Host"), Some(&"127.0.0.1:7878".to_string()));

    }  

    #[test]
    fn test_path_traversal(){
        let request_line = fs::read_to_string("tests/fixtures/path_traversal.txt").expect("couldn't read empty request to string");

        let req_first_line = collect_request_line(&request_line).unwrap();

        let res = parse_request_line(&req_first_line);

        println!("\n\nRESULTS HERE path traversal:\n\n {:?}\n\n", res);

        assert_eq!(res, Some(("GET".to_string(), "/../../etc/passwd".to_string(), "HTTP/1.1".to_string())));
    }

    #[test]
    fn test_uknown_method(){
        let request_line = fs::read_to_string("tests/fixtures/unknown_method.txt").expect("couldn't read empty request to string");

        let req_first_line = collect_request_line(&request_line).unwrap();

        let res = parse_request_line(&req_first_line);

        println!("\n\nRESULTS HERE unknown method:\n\n {:?}\n\n", res);

        assert_eq!(res, Some(("POST".to_string(), "/".to_string(), "HTTP/1.1".to_string())));

        let header_hashmap = collect_headers(&request_line);

        assert_eq!(header_hashmap.len(), 2);

        assert_eq!(header_hashmap.get("Host"), Some(&"127.0.0.1:7878".to_string()));

        assert_eq!(header_hashmap.get("Content-Length"), Some(&"0".to_string()));
    }

    #[test]
    fn test_weird_whitespace(){

        let request_line = fs::read_to_string("tests/fixtures/weird_whitespace.txt").expect("couldn't read weird_whitespace.txt");

        let req_first_line = collect_request_line(&request_line).unwrap();

        let res = parse_request_line(&req_first_line);

        println!("\n\nRESULTS HERE weird whitespace:\n\n {:?}\n\n", res);

        assert_eq!(res, None);

    }

}