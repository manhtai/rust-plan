use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use std::process::exit;

fn main() {
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

    if let Some(matches) = matches.subcommand_matches("set") {
        eprint!("unimplemented");
        exit(1)
    }

    if let Some(matches) = matches.subcommand_matches("get") {
        eprint!("unimplemented");
        exit(1)
    }

    if let Some(matches) = matches.subcommand_matches("rm") {
        eprint!("unimplemented");
        exit(1)
    }

    exit(1)
}

