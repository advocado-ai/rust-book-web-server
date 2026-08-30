use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("failed to bind to 127.0.0.1:7878");

    for stream in listener.incoming(){
        let stream = stream.expect("failed to establish connection");

        handle_connection(stream);
    }

}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&stream);

    //let request_line = buf_reader.lines().next().expect("failed to read request line").expect("failed to read request line");

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.expect("could not read request from client"))
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);


    if http_request[0] == "GET / HTTP/1.1"{
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").expect("failed to read html file for response");
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );
        stream.write_all(response.as_bytes()).expect("failed to write response to TCP stream");
    }else{
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").expect("failed to read 404 html");
        let length = contents.len();

        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).expect("failed to write 404 response to tcp stream");
    }

    /*
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").expect("failed to read html file");

    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).expect("failed to return response");
    */
}
