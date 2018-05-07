/*
 * Implement abstract syntax tree nodes for the python language.
 */

/*
#[derive(Debug)]
pub struct Location {
    pub row: i32,
    pub column: i32,
}

#[derive(Debug)]
pub struct Node {
    pub location: Location,
}
*/

#[derive(Debug)]
pub struct Program {
  pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Break,
    Continue,
    Pass,
    Expression { expression: Expression },
    If { test: Expression, body: Vec<Statement>},
    While { test: Expression, body: Vec<Statement> },
    For { test: Expression },
    ClassDef {
      name: String,
      // TODO: docstring: String,
    },
    FunctionDef {
      name: String,
      // docstring: String,
      body: Vec<Statement>,
    },
}

#[derive(Debug)]
pub enum Expression {
    Binop { a: Box<Expression>, op: Operator, b: Box<Expression> },
    Call { function: Box<Expression>, args: Vec<Expression> },
    Number { value: i32 },
    String { value: String },
    Identifier { name: String },
    True,
    False,
    None,
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Sub,
    Mult,
    MatMult,
    Div,
    Mod,
    Pow,
    LShift,
    RShift,
    BitOr,
    BitXor,
    BitAnd,
    FloorDiv,
}

