
/*
 * Some functions are built into the interpreter, for example the print
 * function is such a builtin function.
 *
 * Inspiration can be found here:
 * https://github.com/python/cpython/blob/master/Python/bltinmodule.c
 */

use std::rc::Rc;
use super::pyobject::PyObject;
use std::io::{self, Write};

pub fn fill_scope() {
  // scope[String::from("print")] = print;
}

pub fn print(args: Vec<Rc<PyObject>>) {
    // println!("Woot: {:?}", args);
    trace!("print called with {:?}", args);
    for a in args {
        print!("{} ", a.str());
    }
    println!();
    io::stdout().flush().unwrap();
}

fn any() {
}

fn all() {
}

