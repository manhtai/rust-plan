use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(b"PING") {
        let mut message = "PONG\n".as_bytes();
        if buffer[5..].iter().any(|x| { *x > 10 }) {
            message = &buffer[5..];
        }
        stream.write(message).unwrap();
        stream.flush().unwrap();
    }
}
