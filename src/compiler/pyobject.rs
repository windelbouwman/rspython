use std::rc::Rc;
use std::ops::{Add, Sub};

#[derive(Debug)]
pub enum PyObject {
    String { value: String },
    Integer { value: i32 },
    RustFunction { function: fn(Vec<Rc<PyObject>>) },
}

impl PyObject {
    pub fn call(&self, args: Vec<Rc<PyObject>>) {
        match *self {
            PyObject::RustFunction { ref function } => {
                function(args);
            },
            _ => {
                println!("Not impl {:?}", self);
                panic!("Not impl");
            }
        }
    }

    pub fn str(&self) -> String {
        match *self {
            PyObject::String { ref value } => { value.clone() },
            PyObject::Integer { ref value } => { format!("{:?}", value) },
            _ => {
                println!("Not impl {:?}", self);
                panic!("Not impl");
            }
        }
    }
}

impl Add for PyObject {
    type Output = PyObject;

    fn add(self, rhs: PyObject) -> Self::Output {
        match self {
            PyObject::Integer { ref value } => {
                let value1 = value;
                match rhs {
                    PyObject::Integer { ref value } => {
                        let value2 = value;
                        PyObject::Integer { value: value1 + value2 }
                    },
                    _ => {
                        panic!("NOT IMPL");
                    }
                }
            },
            _ => {
                // Lookup __add__ method in dictionary?
                panic!("NOT IMPL");
            }
        }
    }
}

impl Sub for PyObject {
    type Output = PyObject;

    fn sub(self, rhs: PyObject) -> Self::Output {
        match self {
            PyObject::Integer { value } => {
                let value1 = value;
                match rhs {
                    PyObject::Integer { value } => {
                        let value2 = value;
                        PyObject::Integer { value: value1 - value2 }
                    },
                    _ => {
                        panic!("NOT IMPL");
                    }
                }
            },
            _ => {
                panic!("NOT IMPL");
            }
        }
    }
}
