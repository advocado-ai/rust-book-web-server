use std::{
    fs,
    thread,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use hello::{ThreadPool, 
    collect_request_line, 
    parse_request_line, 
    collect_headers,
    parse_header_line,
};

//enum files
use hello::response::{Response, ResponseRoute};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind to 127.0.0.1:7878");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        let stream = stream.expect("failed to establish connection");

        pool.execute(||{
            handle_connection(stream);
        });
    }

    println!("Shutting down.");

}



// closure for threads to run
fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&stream);

    // FIXME: outer None means client connected and sent nothing, inner Err means an I/O failure: Option<Result<String, io::Error>>
    //let request_line = buf_reader.lines().next().unwrap().unwrap();

    let request_line = match buf_reader.lines().next(){
        //match outer Option
        Some(result_req_line) => {
            // match inner Result
            match result_req_line{
                Ok(req_line) => req_line,
                Err(_e) => return, //I/O failure
            }
        }
        None => return, //client sent nothing
    };

    
    let resp: (&str, &str) = if let Some((method, path, _version)) = parse_request_line(&request_line){
        match (method.as_str(), path.as_str()){
            ("GET", "/") => (Response::Ok.status_line(), ResponseRoute::Index.filename()),
            ("GET", "/about") => (Response::Ok.status_line(), ResponseRoute::About.filename()) ,
            (m,_) if m != "GET" => (Response::MethodNotAllowed.status_line(), ResponseRoute::MethodNotAllowed.filename()),
            (_, _) => (Response::NotFound.status_line(), ResponseRoute::NotFound.filename()),
        }
    }else{
        (Response::BadRequest.status_line(), ResponseRoute::BadRequest.filename())
    };

    
    let status_line = resp.0;

    let filename = resp.1;

    let contents = fs::read_to_string(filename).expect("failed to read html");

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("failed to send response");
  
}

