use clap::{App, Arg};

fn main() {
    let matches = App::new("My CLI")
        .version("0.1")
        .author("manhtai")
        .arg(Arg::with_name("v")
            .short("v")
            .multiple(true)
            .help("Set verbose level"))
        .get_matches();

    match matches.occurrences_of("v"){
        0 => println!("No verbose!"),
        1 => println!("Level 1 verbose"),
        2 => println!("Level 2 verbose"),
        3 | _ => print!("Level 3 verbose and up")
    }
}