use clap::{App, Arg, SubCommand};

use kvs::{KvsCommand, KvsResult, Result};
use std::io::{Read, Write, BufReader, BufRead};
use std::net::TcpStream;

use std::process::exit;

fn exchange(mut stream: TcpStream, command: &KvsCommand) -> () {
    let data = serde_json::to_string(&command).unwrap();
    writeln!(stream, "{}", data).unwrap();
    stream.flush().unwrap();

    let mut reader = BufReader::new(stream);
    let mut buf = String::new();

    reader.read_line(&mut buf).unwrap();
    let result: KvsResult = serde_json::from_str(&buf).unwrap();

    match result {
        KvsResult::Ok => {
            exit(0)
        }
        KvsResult::Some(value) => {
            println!("{}", value);
            exit(0)
        }
        KvsResult::None => {
            println!("Key not found");
            exit(0)
        }
        KvsResult::Error(e) => {
            eprintln!("{}", e);
            exit(1)
        }
    }
}

fn main() -> Result<()> {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("manhtai")
        .arg(Arg::with_name("V").help("Show package version"))
        .subcommand(
            SubCommand::with_name("get")
                .help("Get value from key")
                .arg(Arg::with_name("key").required(true))
                .arg(Arg::with_name("addr")
                    .long("addr")
                    .takes_value(true)
                    .help("Server address"))
        )
        .subcommand(
            SubCommand::with_name("set")
                .help("Get value from key")
                .arg(Arg::with_name("key").required(true))
                .arg(Arg::with_name("value").required(true))
                .arg(Arg::with_name("addr")
                    .long("addr")
                    .takes_value(true)
                    .help("Server address"))
        )
        .subcommand(
            SubCommand::with_name("rm")
                .help("Remove key")
                .arg(Arg::with_name("key").required(true))
                .arg(Arg::with_name("addr")
                    .long("addr")
                    .takes_value(true)
                    .help("Server address"))
        )
        .get_matches();

    if matches.is_present("V") {
        print!(env!("CARGO_PKG_VERSION"))
    }


    if let Some(matches) = matches.subcommand_matches("set") {
        let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4000");
        let stream = TcpStream::connect(addr).unwrap();
        if let Some(key) = matches.value_of("key") {
            if let Some(value) = matches.value_of("value") {
                exchange(stream, &KvsCommand::Set(key.to_owned(), value.to_owned()))
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4000");
        let stream = TcpStream::connect(addr).unwrap();
        if let Some(key) = matches.value_of("key") {
            exchange(stream, &KvsCommand::Get(key.to_owned()))
        }
    }

    if let Some(matches) = matches.subcommand_matches("rm") {
        let addr = matches.value_of("addr").unwrap_or("127.0.0.1:4000");
        let stream = TcpStream::connect(addr).unwrap();
        if let Some(key) = matches.value_of("key") {
            exchange(stream, &KvsCommand::Remove(key.to_owned()))
        }
    }

    exit(1)
}
