
extern crate clap;
use clap::{Arg, App};
mod parser;

fn main() {
  let matches = App::new("Rupy")
                .version("0.0.1")
                .author("Windel Bouwman")
                .about("Rust implementation of the Python language")
                .arg(Arg::with_name("script")
                     .required(true)
                     .index(1))
                .get_matches();
  let script_file = matches.value_of("script").unwrap_or("foo");
  println!("Running file {}", script_file);
  let ast = parser::parse(script_file);
  println!("Got ast: {}", ast);
}
