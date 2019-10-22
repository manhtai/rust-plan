use std::net::TcpStream;
use std::io::{Write, Read};
use std::str::from_utf8;

fn main() {
    let mut stream = TcpStream::connect("localhost:7878").unwrap();
    let message = "PING".as_bytes();
    stream.write(message).unwrap();

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Server replied: {}", from_utf8(buffer.as_ref()).unwrap());
}
