use clap::{App, Arg, ArgMatches};
use minipython::compiler::CompilerInstance;
use std::path::Path;

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("MiniPython compiler")
        .about("Compiles MiniPython programs")
        .arg(Arg::with_name("out")
            .short("o")
            .long("out")
            .help("Sets the output file name")
            .value_name("FILE")
            .required(true)
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
    let input_file_name = matches.value_of("INPUT").unwrap();
    let input_path = Path::new(input_file_name);
    let output_file_path = matches.value_of("out").map(|f| Path::new(f)).unwrap();
    let mut compiler = CompilerInstance::new(input_path, output_file_path).unwrap();
    match compiler.run() {
        Ok(()) => println!("Compilation successful!"),
        Err(e) => println!("Compilation failed: {}", e)
    }
}
