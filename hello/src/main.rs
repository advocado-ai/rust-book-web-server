use std::{
    fs,
    thread,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

use hello::{ThreadPool, parse_request};

 

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind to 127.0.0.1:7878");

    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2){
        let stream = stream.expect("failed to establish connection");

        pool.execute(||{
            handle_connection(stream);
        });
    }

    println!("Shutting down.");

}

enum Request{
    GetIndex,
    
    Unknown(String),

}

impl Request{
    fn request_path(&self) -> String{
        match self{
            Request::GetIndex => "GET / HTTP/1.1".to_string(),
            Request::Unknown(path)=> path.to_string(),
        }
    }
}

enum Response{
    Ok,
    NotFound,
}

impl Response{
    fn status_line(&self) -> &'static str{
        match self{
            Response::Ok => "HTTP/1.1 200 OK",
            Response::NotFound => "HTTP/1.1 404 NOT FOUND",
        }
    }
    fn filename(&self) -> &'static str{
        match self{
            Response::Ok=>"hello.html",
            Response::NotFound => "404.html",
        }
    }
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&stream);

    //todo: nested match stmts
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    //call parse_request 
    //let status_line = parse_request(request_line);


    let (status_line, filename) = if request_line == Request::GetIndex.request_path() {
        (Response::Ok.status_line(), Response::Ok.filename())
    }else{
        (Response::NotFound.status_line(), Response::NotFound.filename())
    };

    let contents = fs::read_to_string(filename).expect("failed to read html");

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("failed to send response");
  
}

