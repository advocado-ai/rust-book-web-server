use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").expect("TcpListener did not bind to local host port 7878");

    for stream in listener.incoming(){
        let stream = stream.expect("error: stream in listener.incoming");

        println!("Connection established");
    }

}
