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
}

#[derive(Debug)]
pub enum Expression {
    Binop { a: Box<Expression>, op: String, b: Box<Expression> },
    Call { f: Box<Expression>, args: Vec<Expression> },
    Number { value: i32 },
    String { value: String },
    Identifier { name: String },
}

