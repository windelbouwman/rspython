
extern crate clap;
use clap::{Arg, App};
use std::path::Path;
mod parser;


fn main() {
  let matches = App::new("Rupy")
                .version("0.0.1")
                .author("Windel Bouwman")
                .about("Rust implementation of the Python language")
                .arg(Arg::with_name("script")
                     .required(true)
                     .index(1))
                .arg(Arg::with_name("v")
                     .short("v")
                     .multiple(true)
                     .help("Give the verbosity"))
                .get_matches();

  // Figure out the filename:
  let script_file = matches.value_of("script").unwrap_or("foo");
  println!("Running file {}", script_file);

  // Parse an ast from it:
  let filepath = Path::new(script_file);
  match parser::parse(filepath) {
    Ok(program) => {
      println!("Got ast: {:?}", program);
      let bytecode = parser::compile(program);
      println!("Code object: {:?}", bytecode);
      parser::evaluate(bytecode);
    },
    Err(msg) => println!("Parsing went horribly wrong: {}", msg),
  }
}


