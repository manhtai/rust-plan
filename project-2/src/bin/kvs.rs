use std::process::exit;
use std::path::Path;

use clap::{App, Arg, SubCommand};

use kvs::{KvStore, Result};

fn main() -> Result<()> {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("manhtai")
        .arg(Arg::with_name("V")
            .help("Show package version"))
        .subcommand(SubCommand::with_name("get")
            .help("Get value from key")
            .arg(Arg::with_name("key").required(true)))
        .subcommand(SubCommand::with_name("set")
            .help("Get value from key")
            .arg(Arg::with_name("key").required(true))
            .arg(Arg::with_name("value").required(true)))
        .subcommand(SubCommand::with_name("rm")
            .help("Remove key")
            .arg(Arg::with_name("key").required(true)))
        .get_matches();

    if matches.is_present("V") {
        print!(env!("CARGO_PKG_VERSION"))
    }

    let mut store = KvStore::open(Path::new("./target/"))?;

    if let Some(matches) = matches.subcommand_matches("set") {
        if let Some(key) = matches.value_of("key") {
            if let Some(value) = matches.value_of("value") {
                match store.set(key.to_owned(), value.to_owned()) {
                    Err(err) => println!("{:?}", err),
                    _ => ()
                }
                exit(0)
            }
        }
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        if let Some(key) = matches.value_of("key") {
            match store.get(key.to_owned()) {
                Ok(option) => match option {
                    Some(value) => println!("{}", value),
                    _ => ()
                },
                Err(error) => {
                    println!("{:?}", error);
                },
            };
            exit(0)
        }
    }

    if let Some(matches) = matches.subcommand_matches("rm") {
        if let Some(key) = matches.value_of("key") {
            let k = key.to_owned();
            match store.remove(k) {
                Err(error) =>  {
                    println!("{:?}", error);
                    exit(1)
                },
                _ => ()
            };
            exit(0)
        }
    }

    exit(1)
}

