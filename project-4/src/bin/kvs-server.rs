use std::path::Path;

use clap::{App, Arg};

use kvs::{KvsCommand, KvsResult, KvStore, KvsEngine, SledKvsEngine, Result};
use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, TcpStream};
use std::env;

fn exchange(mut stream: TcpStream, store: &mut dyn KvsEngine) -> Result<()> {
    let mut buf = String::new();
    let mut reader = BufReader::new(&stream);
    reader.read_line(&mut buf);
    print!("Receive: {}", buf);

    let command: KvsCommand = serde_json::from_str(&buf).unwrap();

    let result = match command {
        KvsCommand::Set(key, value) => match store.set(key.to_owned(), value.to_owned()) {
            Err(e) => KvsResult::Error(e),
            _ => KvsResult::Ok,
        },
        KvsCommand::Remove(key) => match store.remove(key) {
            Err(e) => KvsResult::Error(e),
            _ => KvsResult::Ok,
        },
        KvsCommand::Get(key) => match store.get(key) {
            Ok(v) => match v {
                Some(value) => KvsResult::Some(value),
                None => KvsResult::None,
            },
            Err(e) => KvsResult::Error(e),
        },
    };

    writeln!(stream, "{}", serde_json::to_string(&result).unwrap()).unwrap();
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
        kvs_ = KvStore::open(Path::new("."))?;
        store = &mut kvs_;
    } else {
        sled_ = SledKvsEngine::open(Path::new("."))?;
        store = &mut sled_;
    }

    let addr = matches.value_of("address").unwrap_or("127.0.0.1:4000");
    let listener = TcpListener::bind(addr).unwrap();

    eprintln!(env!("CARGO_PKG_VERSION"));
    eprintln!("Server listen in: {} with engine: {}", addr, engine);

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        exchange(stream, store);
    }

    Ok(())
}
