use clap::{App, Arg, ArgMatches};
use minipython::compiler::CompilerInstance;
use std::path::Path;

fn parse_args<'a>() -> ArgMatches<'a> {
    App::new("MiniPython compiler")
        .about("Compiles MiniPython programs")
        .arg(Arg::with_name("OUT")
            .short("o")
            .long("out")
            .help("Sets the output file name")
            .value_name("FILE")
            .takes_value(true))
        .arg(Arg::with_name("EXE")
            .help("Create executeable")
            .long("exe")
            .value_name("EXECUTABLE_FILE")
            .takes_value(true))
        .arg(Arg::with_name("INPUT")
            .help("Input file")
            .required(true)
            .value_name("FILE")
            .index(1))
        .get_matches()
}

fn compile(matches: ArgMatches) -> Result<(), String> {
    let input_path = Path::new(matches.value_of("INPUT").unwrap());
    let default_output = input_path.with_extension("c");
    let output_file_path = matches
        .value_of("OUT")
        .map(Path::new)
        .unwrap_or_else(|| default_output.as_path());
    let compile = matches
        .value_of("EXE")
        .map(Path::new);
    let mut compiler = CompilerInstance::new(input_path, output_file_path, compile)?;
    compiler.run()
}

fn main() {
    let matches = parse_args();
    match compile(matches) {
        Ok(()) => println!("Compilation successful!"),
        Err(e) => println!("Compilation failed: {}", e)
    }
}
