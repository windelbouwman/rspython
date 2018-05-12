use std::rc::Rc;
use std::ops::{Add, Sub, Mul};

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

impl<'a> Add<&'a PyObject> for &'a PyObject {
    type Output = PyObject;

    fn add(self, rhs: &'a PyObject) -> Self::Output {
        match self {
            &PyObject::Integer { ref value } => {
                let value1 = value;
                match rhs {
                    &PyObject::Integer { ref value } => {
                        let value2 = value;
                        PyObject::Integer { value: value1 + value2 }
                    },
                    _ => {
                        panic!("NOT IMPL");
                    }
                }
            },
            _ => {
                // TODO: Lookup __add__ method in dictionary?
                panic!("NOT IMPL");
            }
        }
    }
}

impl<'a> Sub<&'a PyObject> for &'a PyObject {
    type Output = PyObject;

    fn sub(self, rhs: &'a PyObject) -> Self::Output {
        match self {
            &PyObject::Integer { value } => {
                let value1 = value;
                match rhs {
                    &PyObject::Integer { value } => {
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

impl<'a> Mul<&'a PyObject> for &'a PyObject {
    type Output = PyObject;

    fn mul(self, rhs: &'a PyObject) -> Self::Output {
        match self {
            &PyObject::Integer { value } => {
                let value1 = value;
                match rhs {
                    &PyObject::Integer { value } => {
                        let value2 = value;
                        PyObject::Integer { value: value1 * value2 }
                    },
                    _ => {
                        panic!("NOT IMPL");
                    }
                }
            },
            &PyObject::String { ref value } => {
                let value1 = value;
                match rhs {
                    &PyObject::Integer { value } => {
                        let value2 = value;
                        let mut result = String::new();
                        for _x in 0..value2 {
                            result.push_str(value1.as_str());
                        }
                        PyObject::String { value: result }
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

#[cfg(test)]
mod tests {
    use super::PyObject;

    #[test]
    fn test_add_py_integers() {
        let a = PyObject::Integer { value: 33 };
        let b = PyObject::Integer { value: 12 };
        let c = &a + &b;
        match c {
            PyObject::Integer { value } => {
                assert_eq!(value, 45)
            },
            _ => { assert!(false) }
        }
    }

    #[test]
    fn test_multiply_str() {
        let a = PyObject::String { value: String::from("Hello ") };
        let b = PyObject::Integer { value: 4 };
        let c = &a * &b;
        match c {
            PyObject::String { value } => {
                assert_eq!(value, String::from("Hello Hello Hello Hello "))
            },
            _ => { assert!(false) }
        }
    }

}
