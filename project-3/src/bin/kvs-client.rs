use clap::{App, Arg, SubCommand};

use kvs::{Command, Result};
use std::io::{Read, Write};
use std::net::TcpStream;

use std::process::exit;

fn exchange(stream: &mut TcpStream, command: &Command) -> Result<String> {
    let data = serde_json::to_string(&command).unwrap();
    stream.write_all(data.as_bytes()).unwrap();
    let mut buf = String::new();
    stream.read_to_string(&mut buf).unwrap();
    Ok(buf.to_string())
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
                    .value_name("address")
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
                    .value_name("address")
                    .help("Server address"))
        )
        .subcommand(
            SubCommand::with_name("rm")
                .help("Remove key")
                .arg(Arg::with_name("key").required(true))
                .arg(Arg::with_name("addr")
                    .long("addr")
                    .takes_value(true)
                    .value_name("address")
                    .help("Server address"))
        )
        .get_matches();

    if matches.is_present("V") {
        print!(env!("CARGO_PKG_VERSION"))
    }


    if let Some(matches) = matches.subcommand_matches("set") {
        let addr = matches.value_of("address").unwrap_or("127.0.0.1:4000");
        let mut stream = TcpStream::connect(addr).unwrap();
        if let Some(key) = matches.value_of("key") {
            if let Some(value) = matches.value_of("value") {
                if let Err(err) =
                exchange(&mut stream, &Command::Set(key.to_owned(), value.to_owned()))
                {
                    println!("{:?}", err)
                }
                exit(0)
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        let addr = matches.value_of("address").unwrap_or("127.0.0.1:4000");
        let mut stream = TcpStream::connect(addr).unwrap();
        if let Some(key) = matches.value_of("key") {
            match exchange(&mut stream, &Command::Get(key.to_owned())) {
                Ok(value) => {
                    println!("{}", value);
                }
                Err(error) => {
                    println!("{:?}", error);
                }
            };
            exit(0)
        }
    }

    if let Some(matches) = matches.subcommand_matches("rm") {
        let addr = matches.value_of("address").unwrap_or("127.0.0.1:4000");
        let mut stream = TcpStream::connect(addr).unwrap();
        if let Some(key) = matches.value_of("key") {
            if let Err(error) = exchange(&mut stream, &Command::Get(key.to_owned())) {
                println!("{:?}", error);
                exit(1)
            };
            exit(0)
        }
    }

    exit(1)
}
