use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use hello::ThreadPool;
use hello::response::Response;
use hello::router::route;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("failed to bind to 127.0.0.1:7878");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming(){
        let stream = stream.expect("failed to establish connection");
        
        pool.execute(||{
            handle_connection(stream);
        });
    
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        Some(Err(_)) | None => return,
    };

    let response = route(&request_line);

    let http = build_http_response(&response);

    if let Err(e) = stream.write_all(http.as_bytes()) {
        eprintln!("failed to send response: {e}");
    }
}

fn build_http_response(response: &Response) -> String {
    let contents = match fs::read_to_string(response.filename()) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("failed to read {}: {e}", response.filename());
            return "HTTP/1.1 500 INTERNAL SERVER ERROR\r\nContent-Length: 0\r\n\r\n".to_string();
        }
    };

    let length = contents.len();

    format!(
        "{}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}", response.status_line()
    )
}