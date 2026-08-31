use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind to 127.0.0.1:7878");

    for stream in listener.incoming(){
        let stream = stream.expect("failed to establish connection");

        handle_connection(stream);
    }

}

enum Request{
    GetIndex,
    GetSleep,
    Unknown(String),

}

impl Request{
    fn request_path(&self) -> String{
        match self{
            Request::GetIndex => "GET / HTTP/1.1".to_string(),
            Request::GetSleep => "GET /sleep HTTP/1.1".to_string(),
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
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        s if s == Request::GetIndex.request_path() => (Response::Ok.status_line(), Response::Ok.filename()),
        s if s == Request::GetSleep.request_path() => {
            thread::sleep(Duration::from_secs(5));
            (Response::Ok.status_line(), Response::Ok.filename())
        },
        _ => (Response::NotFound.status_line(), Response::NotFound.filename()),
    };


    let contents = fs::read_to_string(filename).expect("failed to read html");

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("failed to send response");
   
}
