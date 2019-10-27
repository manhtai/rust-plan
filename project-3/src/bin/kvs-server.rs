use std::path::Path;

use clap::{App, Arg};

use kvs::{Command, KvStore, KvsEngine, SledKvsEngine, Result};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};


fn exchange(stream: &mut TcpStream, store: &mut dyn KvsEngine) -> Result<()> {
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    let command: Command = serde_json::from_str(&buf).unwrap();

    let default = "".to_owned();
    let result = match command {
        Command::Set(key, value) => match store.set(key.to_owned(), value.to_owned()) {
            Err(e) => e.to_string(),
            _ => default,
        },
        Command::Remove(key) => match store.remove(key) {
            Err(e) => e.to_string(),
            _ => default,
        },
        Command::Get(key) => match store.get(key) {
            Ok(v) => match v {
                Some(value) => value,
                _ => default,
            },
            Err(e) => e.to_string(),
        },
    };

    stream.write_all(result.as_bytes()).unwrap();
    stream.flush().unwrap();
    Ok(())
}

fn main() -> Result<()> {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("manhtai")
        .arg(Arg::with_name("V").help("Show package version"))
        .arg(Arg::with_name("address")
            .long("addr")
            .help("Server address")
            .takes_value(true)
            .value_name("address")
        )
        .arg(Arg::with_name("engine")
            .long("engine")
            .help("KV engine")
            .takes_value(true)
            .value_name("engine")
        )
        .get_matches();

    if matches.is_present("V") {
        print!(env!("CARGO_PKG_VERSION"))
    }

    let engine = matches.value_of("engine").unwrap_or("kvs");
    let mut kvs_;
    let mut sled_;
    let mut store: &mut dyn KvsEngine;
    if engine == "kvs" {
        kvs_ = KvStore::open(Path::new("."));
        store = &mut kvs_;
    } else {
        sled_ = SledKvsEngine::open(Path::new("."));
        store = &mut sled_;
    }

    let addr = matches.value_of("address").unwrap_or("127.0.0.1:4000");
    println!("Server listen in: {}", addr);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        exchange(&mut stream, store);
    }

    Ok(())
}