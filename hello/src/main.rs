use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use hello::ThreadPool;
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

    let status_line = response.status_line();
    let contents = fs::read_to_string(response.filename())
    .expect("failed to read html file");
    let length= contents.len();
    
    let http = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(http.as_bytes()).expect("failed to send response");
}