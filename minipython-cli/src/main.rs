use clap::{App, Arg, ArgMatches};

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("MiniPython compiler")
        .about("Compiles MiniPython programs")
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .help("Sets the output file name")
            .value_name("FILE")
            .required(false)
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Input file")
            .required(true)
            .value_name("FILE")
            .index(1))
        .get_matches()
}

fn main() {
    let matches = parse_args();
    println!("Hello, world!");
}
