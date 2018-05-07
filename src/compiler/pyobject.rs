use std::rc::Rc;

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
                panic!("Not impl");
            }
        }
    }

    pub fn str(&self) -> String {
        match *self {
            PyObject::String { ref value } => { value.clone() },
            _ => {
                panic!("Not impl");
            }
        }
    }
}
