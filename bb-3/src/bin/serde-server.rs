use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use bb::RedisPing;

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

    let ping: RedisPing = bincode::deserialize(buffer.as_ref()).unwrap();
    let mut message = ping.message.trim();

    if message.is_empty() {
        message = "PONG";
    }
    stream.write(message.as_bytes()).unwrap();
    stream.flush().unwrap();
}
