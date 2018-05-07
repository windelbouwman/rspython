
/*
 * Implement python as a virtual machine with bytecodes.
 */

/*
let load_const_string = 0x16;
let call_function = 0x64;
*/


/*
 * Primitive instruction type, which can be encoded and decoded.
 */

#[derive(Debug)]
pub struct CodeObject {
  pub instructions: Vec<Instruction>,
}

impl CodeObject {
    pub fn new() -> CodeObject {
        CodeObject {
            instructions: Vec::new(),
        }
    }
}

#[derive(Debug)]
pub enum Instruction {
    LoadName { name: String },
    LoadConst { value: i32 },
    LoadStringConstant { value: String },
    Pop,
    Pass,
    Continue,
    Break,
    CallFunction,
    ReturnValue,
}


