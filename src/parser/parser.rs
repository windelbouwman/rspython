use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use parser::python;
use parser::ast;

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

      match python::parse_Program(&txt) {
        Err(why) => Err(String::from(format!("{:?}", why))),
        Ok(p) => Ok(p),
      }
    },
    Err(msg) => Err(msg),
  }
}
