
/*
 * Implement virtual machine to run instructions.
 */

use std::collections::HashMap;
use std::cell::RefMut;
use std::ops::Deref;

use super::bytecode;
use super::builtins;
use super::pyobject::{PyObject, PyObjectRef};

// use objects::objects;

// Container of the virtual machine state:
pub fn evaluate(code: bytecode::CodeObject) {
    let mut vm = VirtualMachine::new();

    // Register built in function:
    vm.scope.insert(String::from("print"), PyObject::RustFunction { function: builtins::print }.into_ref());

    // { stack: Vec::new() };
    vm.run(code);
}

// Objects are live when they are on stack, or referenced by a name (for now)

struct VirtualMachine {
    stack: Vec<PyObjectRef>,
    scope: HashMap<String, PyObjectRef>,
    program_counter: usize,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            scope: HashMap::new(),
            program_counter: 0,
        }
    }

    fn run(&mut self, code: bytecode::CodeObject) {
        for instruction in code.instructions {
            let instruction2 = instruction;
            self.execute_instruction(instruction2);
        }
    }

    // Execute a single instruction:
    fn execute_instruction(&mut self, instruction: bytecode::Instruction) {
        trace!("Executing instruction: {:?} (stacksize={:?}", instruction, self.stack.len());
        match instruction {
            bytecode::Instruction::LoadStringConstant { value } => {
                let obj = PyObject::String { value }.into_ref();
                self.stack.push(obj.clone()); // Put reference on stack
            },
            bytecode::Instruction::LoadConst { value } => {
                let obj = PyObject::Integer { value }.into_ref();
                self.stack.push(obj.clone());
            },
            bytecode::Instruction::LoadName { name } => {
                // Lookup name in scope and put it onto the stack!
                let obj = &self.scope[&name];
                self.stack.push(obj.clone());
            },
            bytecode::Instruction::StoreName { name } => {
                // take top of stack and assign in scope:
                let obj = self.stack.pop().unwrap();
                self.scope.insert(name, obj);
            },
            bytecode::Instruction::Pop => {
                // Pop value from stack and ignore.
                self.stack.pop();
            },
            bytecode::Instruction::BuildList { size } => {
                let mut elements = Vec::new();
                for _x in 0..size {
                    let obj = self.stack.pop().unwrap();
                    elements.push(obj);
                }
                let list_obj = PyObject::List { elements: elements }.into_ref();
                self.stack.push(list_obj);
            },
            bytecode::Instruction::BuildTuple { size } => {
                let mut elements = Vec::new();
                for _x in 0..size {
                    let obj = self.stack.pop().unwrap();
                    elements.push(obj);
                }
                let list_obj = PyObject::Tuple { elements: elements }.into_ref();
                self.stack.push(list_obj);
            },
            bytecode::Instruction::BuildMap { size } => {
                let mut elements = Vec::new();
                for _x in 0..size {
                    let obj = self.stack.pop().unwrap();
                    elements.push(obj);
                }
                let list_obj = PyObject::Tuple { elements: elements }.into_ref();
                self.stack.push(list_obj);
            },
            bytecode::Instruction::BinaryPower => {
            },
            bytecode::Instruction::BinaryMultiply => {
                // Pop value from stack and ignore.
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = (&*a.borrow() * &*b.borrow()).into_ref();
                self.stack.push(result);
            },
            bytecode::Instruction::BinaryMatrixMultiply => {
                // Pop value from stack and ignore.
                let b = &*self.stack.pop().unwrap();
                let a = &*self.stack.pop().unwrap();
                // let result = Rc::new(a * b);
                // self.stack.push(result);
            },
            bytecode::Instruction::BinaryDivide => {
            },
            bytecode::Instruction::BinaryFloorDivide => {
            },
            bytecode::Instruction::BinaryModulo => {
            },
            bytecode::Instruction::BinaryAdd => {
                // Pop value from stack and ignore.
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = (&*a.borrow() + &*b.borrow()).into_ref();
                self.stack.pop();
            },
            bytecode::Instruction::BinarySubtract => {
                // Pop value from stack and ignore.
                let b = self.stack.pop().unwrap();
                let a = self.stack.pop().unwrap();
                let result = (&*a.borrow() - &*b.borrow()).into_ref();
                self.stack.push(result);
            },
            bytecode::Instruction::BinaryLshift => {
                let a = &*self.stack.pop().unwrap();
                let b = &*self.stack.pop().unwrap();

            },
            bytecode::Instruction::BinaryRshift => {
                let a = &*self.stack.pop().unwrap();
                let b = &*self.stack.pop().unwrap();

            },
            bytecode::Instruction::BinaryAnd => {
                let a = &*self.stack.pop().unwrap();
                let b = &*self.stack.pop().unwrap();

            },
            bytecode::Instruction::BinaryXor => {
                let a = &*self.stack.pop().unwrap();
                let b = &*self.stack.pop().unwrap();

            },
            bytecode::Instruction::BinaryOr => {
                let a = &*self.stack.pop().unwrap();
                let b = &*self.stack.pop().unwrap();

            },
            bytecode::Instruction::ReturnValue => {
                self.stack.pop();
            },
            bytecode::Instruction::GetIter => {
                let iterated_obj = self.stack.pop().unwrap();
                let iter_obj = PyObject::Iterator {
                    position: 0, iterated_obj: iterated_obj
                }.into_ref();
                self.stack.push(iter_obj);
            },
            bytecode::Instruction::ForIter => {

                let next_obj: Option<PyObjectRef> = {
                    let top_of_stack = self.stack.last().unwrap();
                    let mut ref_mut: RefMut<PyObject> = top_of_stack.deref().borrow_mut();
                    // We require a mutable pyobject here to update the iterator:
                    let mut iterator = ref_mut; // &mut PyObject = ref_mut.;
                    // let () = iterator;
                    iterator.nxt()
                };
                match next_obj {
                    Some(v) => {
                        self.stack.push(v);
                    },
                    None => {
                        // End of for loop
                        // TODO: jmp
                        self.program_counter = 0;
                    }
                }
            },
            bytecode::Instruction::CallFunction { count } => {
                let mut args: Vec<PyObjectRef> = Vec::new();

                for _x in 0..count {
                    args.push(self.stack.pop().unwrap());
                }

                args.reverse();
                let func_ref = self.stack.pop().unwrap();
                let f=func_ref.borrow();// = &*func_ref.borrow();
                f.call(args);
                // call_stack.push();
                // If a builtin function, then call directly, otherwise, execute it?
                // execute(function.code);
            },
            bytecode::Instruction::Break => {

            },
            bytecode::Instruction::Pass => {
                // Ah, this is nice, just relax!
            },
            bytecode::Instruction::Continue => {

            },
        }
    }
}
