
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
    // A stack of block to keep track of continue break locations.
    block_stack: Vec<(bytecode::Label, bytecode::Label)>,
    scope: HashMap<String, PyObjectRef>,
    program_counter: usize,
}

impl VirtualMachine {
    fn new() -> VirtualMachine {
        VirtualMachine {
            stack: Vec::new(),
            block_stack: Vec::new(),
            scope: HashMap::new(),
            program_counter: 0,
        }
    }

    fn run(&mut self, code: bytecode::CodeObject) {
        // Process instruction at the program counter until exception or finished.
        loop {
            // for instruction in code.instructions {
            if self.program_counter > code.instructions.len() {
                break;
            }

            let ref instruction = code.instructions[self.program_counter];
            trace!("Executing instruction: {:?} (stacksize={:?}, pc={:?}/{:?}", instruction, self.stack.len(), self.program_counter, code.instructions.len());
            self.program_counter += 1;
            self.execute_instruction(&instruction);
        }
    }

    // Execute a single instruction:
    fn execute_instruction(&mut self, instruction: &bytecode::Instruction) {
        // trace!("Executing instruction: {:?} (stacksize={:?}", instruction, self.stack.len());
        match instruction {
            &bytecode::Instruction::LoadStringConstant { ref value } => {
                let obj = PyObject::String { value: value.clone() }.into_ref();
                self.stack.push(obj.clone()); // Put reference on stack
            },
            &bytecode::Instruction::LoadConst { ref value } => {
                let obj = PyObject::Integer { value: *value }.into_ref();
                self.stack.push(obj.clone());
            },
            &bytecode::Instruction::LoadName { ref name } => {
                // Lookup name in scope and put it onto the stack!
                let obj = &self.scope[name];
                self.stack.push(obj.clone());
            },
            &bytecode::Instruction::StoreName { ref name } => {
                // take top of stack and assign in scope:
                let obj = self.stack.pop().unwrap();
                self.scope.insert(name.clone(), obj);
            },
            &bytecode::Instruction::Pop => {
                // Pop value from stack and ignore.
                self.stack.pop();
            },
            &bytecode::Instruction::BuildList { size } => {
                let mut elements = Vec::new();
                for _x in 0..size {
                    let obj = self.stack.pop().unwrap();
                    elements.push(obj);
                }
                let list_obj = PyObject::List { elements: elements }.into_ref();
                self.stack.push(list_obj);
            },
            &bytecode::Instruction::BuildTuple { size } => {
                let mut elements = Vec::new();
                for _x in 0..size {
                    let obj = self.stack.pop().unwrap();
                    elements.push(obj);
                }
                let list_obj = PyObject::Tuple { elements: elements }.into_ref();
                self.stack.push(list_obj);
            },
            &bytecode::Instruction::BuildMap { size } => {
                let mut elements = Vec::new();
                for _x in 0..size {
                    let obj = self.stack.pop().unwrap();
                    elements.push(obj);
                }
                let list_obj = PyObject::Tuple { elements: elements }.into_ref();
                self.stack.push(list_obj);
            },
            &bytecode::Instruction::BinaryOperation { ref op } => {
                self.execute_binop(op);
            },
            &bytecode::Instruction::ReturnValue => {
                self.stack.pop();
            },
            &bytecode::Instruction::PushBlock { start, end } => {
                self.block_stack.push((start, end));
            },
            &bytecode::Instruction::PopBlock => {
                self.block_stack.pop();
            }
            &bytecode::Instruction::GetIter => {
                let iterated_obj = self.stack.pop().unwrap();
                let iter_obj = PyObject::Iterator {
                    position: 0, iterated_obj: iterated_obj
                }.into_ref();
                self.stack.push(iter_obj);
            },
            &bytecode::Instruction::ForIter => {
                // The top of stack contains the iterator, lets push it forward:
                let next_obj: Option<PyObjectRef> = {
                    let top_of_stack = self.stack.last().unwrap();
                    let mut ref_mut: RefMut<PyObject> = top_of_stack.deref().borrow_mut();
                    // We require a mutable pyobject here to update the iterator:
                    let mut iterator = ref_mut; // &mut PyObject = ref_mut.;
                    // let () = iterator;
                    iterator.nxt()
                };

                // Check the next object:
                match next_obj {
                    Some(v) => {
                        self.stack.push(v);
                    },
                    None => {
                        // End of for loop
                        let end_label = self.block_stack.last().unwrap().1;
                        self.jump(end_label);
                    }
                }
            },
            &bytecode::Instruction::CallFunction { count } => {
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
            &bytecode::Instruction::Break => {
                let end_label = self.block_stack.last().unwrap().1;
                self.jump(end_label);
            },
            &bytecode::Instruction::Pass => {
                // Ah, this is nice, just relax!
            },
            &bytecode::Instruction::Continue => {
                let start_label = self.block_stack.last().unwrap().0;
                self.jump(start_label);
            },
        }
    }

    fn jump(&mut self, label: bytecode::Label) {
        // self.program_counter = self.label_map[end_label];
    }

    fn execute_binop(&mut self, op: &bytecode::BinaryOperator) {
        let b_ref = self.stack.pop().unwrap();
        let a_ref = self.stack.pop().unwrap();
        let b = &*b_ref.borrow();
        let a = &*a_ref.borrow();
        let result = match op {
            &bytecode::BinaryOperator::Subtract => a - b,
            &bytecode::BinaryOperator::Add => a + b,
            &bytecode::BinaryOperator::Multiply => a * b,
            _ => panic!("NOT IMPL"),
        };
        self.stack.push(result.into_ref());
    }
}
