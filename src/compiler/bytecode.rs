
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
    StoreName { name: String },
    LoadConst { value: i32 },
    LoadStringConstant { value: String },
    BinaryPower,
    BinaryMultiply,
    BinaryMatrixMultiply,
    BinaryDivide,
    BinaryFloorDivide,
    BinaryModulo,
    BinaryAdd,
    BinarySubtract,
    BinaryLshift,
    BinaryRshift,
    BinaryAnd,
    BinaryXor,
    BinaryOr,
    Pop,
    GetIter,
    Pass,
    Continue,
    Break,
    CallFunction { count: usize },
    ForIter,
    ReturnValue,
    BuildTuple { size: usize },
    BuildList { size: usize },
    BuildMap { size: usize },
}


