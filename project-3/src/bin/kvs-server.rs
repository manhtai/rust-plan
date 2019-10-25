use std::path::Path;

use clap::{App, Arg, SubCommand};

use kvs::{Command, KvStore, Result};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn exchange(stream: &mut TcpStream, store: &mut KvStore) -> Result<()> {
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
        .subcommand(
            SubCommand::with_name("--addr")
                .help("Server address")
                .arg(Arg::with_name("address")),
        )
        .subcommand(
            SubCommand::with_name("--engine")
                .help("KV engine")
                .arg(Arg::with_name("name")),
        )
        .get_matches();

    if matches.is_present("V") {
        print!(env!("CARGO_PKG_VERSION"))
    }

    let mut store = KvStore::open(Path::new("."))?;

    let addr = matches.value_of("--addr").unwrap_or("127.0.0.1:4000");
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        exchange(&mut stream, &mut store);
    }

    Ok(())
}
