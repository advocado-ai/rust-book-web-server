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
use hello::response::Response;
use hello::request::Request;

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
                Err(e) => return, //I/O failure
            }
        }
        None => return, //client sent nothing
    };

    // TODO: call collect_request_line
    //  should be result throw error and return?
    let request_line_str = match collect_request_line(&request_line){
        Some(well_formed_request_line) => well_formed_request_line,
        None => return //request_line missing method, path or version, cannot respond
    };

    //resp: (status line, filename)
    let resp: (&str, &str) = match parse_request_line(&request_line_str){
        Some((method, path, version)) => {
            if method == "GET" && path == "/"{
                (Response::Ok_200.status_line(), Response::Ok_200.filename())
            }else if method != "GET"{
                (Response::MethodNotAllowed_405.status_line(), Response::MethodNotAllowed_405.filename())
            }else{
                (Response::NotFound_404.status_line(), Response::NotFound_404.filename())
            }
        },
        None => (Response::BadRequest_400.status_line(), Response::BadRequest_400.filename())
    };


    //let status_line = parse_request(request_line);
    let (status_line, filename) = if request_line == Request::GetIndex.request_path() {
        (Response::Ok_200.status_line(), Response::Ok_200.filename())
    }else{
        (Response::NotFound_404.status_line(), Response::NotFound_404.filename())
    };

    let contents = fs::read_to_string(filename).expect("failed to read html");

    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).expect("failed to send response");
  
}

