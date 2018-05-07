
/*
 * Some functions are built into the interpreter, for example the print
 * function is such a builtin function.
 *
 * Inspiration can be found here:
 * https://github.com/python/cpython/blob/master/Python/bltinmodule.c
 */

use std::rc::Rc;
use compiler::pyobject::PyObject;

pub fn fill_scope() {
  // scope[String::from("print")] = print;
}

pub fn print(args: Vec<Rc<PyObject>>) {
    // println!("Woot: {:?}", args);
    for a in args {
        print!("{}", a.str());
    }
}


fn any() {
}

fn all() {
}

