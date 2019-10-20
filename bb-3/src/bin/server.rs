use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

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

    println!("Client sent: {}", from_utf8(buffer.as_ref()).unwrap());

    if buffer.starts_with(b"PING") {
        let mut message = "PONG\n".as_bytes();
        if buffer[5..].iter().any(|x| { *x > 10 }) {
            message = &buffer[5..];
        }
        stream.write(message).unwrap();
        stream.flush().unwrap();
    }
}
