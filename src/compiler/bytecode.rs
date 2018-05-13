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

pub type Label = usize;

#[derive(Debug)]
pub enum Instruction {
    LoadName { name: String },
    StoreName { name: String },
    LoadConst { value: i32 },
    LoadStringConstant { value: String },
    BinaryOperation { op: BinaryOperator },
    Pop,
    GetIter,
    Pass,
    Continue,
    Break,
    CallFunction { count: usize },
    ForIter,
    ReturnValue,
    PushBlock { start: Label, end: Label },
    PopBlock,
    BuildTuple { size: usize },
    BuildList { size: usize },
    BuildMap { size: usize },
}

#[derive(Debug)]
pub enum BinaryOperator {
    Power,
    Multiply,
    MatrixMultiply,
    Divide,
    FloorDivide,
    Modulo,
    Add,
    Subtract,
    Lshift,
    Rshift,
    And,
    Xor,
    Or,
}

/*
Maintain a stack of blocks on the VM.
pub enum BlockType {
    Loop,
    Except,
}
*/
