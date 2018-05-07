use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use compiler::python;
use compiler::ast;

fn read_file(filename: &Path) -> Result<String, String> {
  match File::open(&filename) {
    Ok(mut file) => {
      let mut s = String::new();

      match file.read_to_string(&mut s) {
        Err(why) => Err(String::from("Reading file failed: ") + why.description()),
        Ok(_) => Ok(s),
      }
    },
    Err(why) => Err(String::from("Opening file failed: ") + why.description()),
  }
}

/*
 * Parse python code.
 * Grammar may be inspired by antlr grammar for python:
 * https://github.com/antlr/grammars-v4/tree/master/python3
 */

pub fn parse(filename: &Path) -> Result<ast::Program, String> {
  println!("Parsing: {}", filename.display());
  match read_file(filename) {
    Ok(txt) => {
      println!("Read contents of file: {}", txt);
      parse_source(&txt)
    },
    Err(msg) => Err(msg),
  }
}

pub fn parse_source(source: &String) -> Result<ast::Program, String> {
      match python::ProgramParser::new().parse(&source) {
        Err(why) => Err(String::from(format!("{:?}", why))),
        Ok(p) => Ok(p),
      }
}

#[test]
fn test_parse_print_hello() {
    let source = String::from(r"print('Hello world')\n");
    parse_source(&source).unwrap();
}

#[test]
fn test_parse_print_2() {
    let source = String::from(r"print('Hello world', 2)\n");
    parse_source(&source).unwrap();
}
