mod util;
mod lexer;
mod parser;
mod emitter;

use std::process::exit;
use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::env;

fn process_input_file(filename: &String) {
    let mut contents = String::new();

    {
        let f = File::open(filename);

        if let Err(ref _e) = f {
            println!("There was error opening the specified input file, aborting.");
            exit(1)
        }

        let mut buf_reader = BufReader::new(f.unwrap());

        if let Err(ref _e) = buf_reader.read_to_string(&mut contents) {
            println!("There was an error reading from the file, aborting.");
            exit(1)
        }
    }

    let lexer = lexer::Lexer::new(&contents);
    let mut parser = parser::Parser { lexer };
    let parsed = parser.parse_top_level();

    if let Err(err) = parsed {
        // TODO implement display
        println!("{:?}", err);
        exit(1)
    }

    let emission = emitter::emit(parsed.unwrap());
    if let Err(err) = emission {
        println!("{:?}", err);
        exit(1)
    }

    let f = File::create("out.js");
    if let Err(ref _e) = f {
        println!("There was error opening the output file, aborting.");
        exit(1)
    }

    if let Err(ref _e) = f.unwrap().write_all(emission.unwrap().as_bytes()){
        println!("There was an error writing to the output file, aborting.");
    } else {
        println!("Output written successfully to out.js");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => process_input_file(&args[1]),
        _ => println!("Usage: cargo run filename"),
    }
}
