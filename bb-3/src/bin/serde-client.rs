use std::net::TcpStream;
use std::io::{Read, Write};
use std::str::from_utf8;
use bincode;

use bb::RedisPing;

fn main() {
    let mut stream = TcpStream::connect("localhost:7878").unwrap();
    let ping = RedisPing { message: "".to_owned() };

    stream.write(bincode::serialize(&ping).unwrap().as_slice()).unwrap();

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    println!("Server replied: {}", from_utf8(buffer.as_ref()).unwrap());
}
