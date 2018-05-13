
/*
 * Take an AST and transform it into bytecode
 */

use super::ast;
use super::bytecode::{Instruction, CodeObject};

struct Compiler {
  codeobject: CodeObject,
}

pub fn compile(p: ast::Program) -> CodeObject {
    let mut compiler = Compiler {
        codeobject: CodeObject::new(),
    };

    compiler.compile_program(p);
    compiler.codeobject
}

impl Compiler {
    fn compile_program(&mut self, program: ast::Program) {
        self.compile_statements(program.statements);
    }

    fn compile_statements(&mut self, statements: Vec<ast::Statement>) {
        for statement in statements {
            self.compile_statement(statement)
        }
    }

    fn compile_statement(&mut self, statement: ast::Statement) {
        trace!("Compiling {:?}", statement);
        match statement {
            ast::Statement::Import { name } => {

            },
            ast::Statement::Expression { expression } => {
                self.compile_expression(expression);

                // Pop result of stack, since we not use it:
                self.emit(Instruction::Pop);
            },
            ast::Statement::If { test, body } => {
                self.compile_expression(test);
                self.compile_statements(body);
            },
            ast::Statement::While { test, body } => {
                self.compile_expression(test);
                self.compile_statements(body);
            },
            ast::Statement::With { items, body } => {
                // TODO
            },
            ast::Statement::For { target, iter, body, or_else } => {
                // The thing iterated:
                for i in iter {
                    self.compile_expression(i);
                }

                // Retrieve iterator
                self.emit(Instruction::GetIter);

                // Start loop
                self.emit(Instruction::ForIter);

                // Start of loop iteration, set targets:
                for t in target {
                    match t {
                        ast::Expression::Identifier { name } => {
                            self.emit(Instruction::StoreName { name: name });
                        },
                        _ => panic!("Not impl"),
                    }
                }

                self.compile_statements(body);
            },
            ast::Statement::FunctionDef { name, body } => {
                self.compile_statements(body);
            },
            ast::Statement::ClassDef { name } => {
                // TODO?
            },
            ast::Statement::Assert { test, msg } => {
                // TODO: if some flag, ignore all assert statements!

                self.compile_expression(test);

                // if true, jump over raise:

                self.emit(Instruction::LoadName { name: String::from("AssertionError") });
                match msg {
                    Some(e) => {
                        self.compile_expression(e);
                        self.emit(Instruction::CallFunction {count: 1});
                    },
                    None => {
                        self.emit(Instruction::CallFunction {count: 0});
                    },
                }
                // TODO?
            },
            ast::Statement::Break => {
                self.emit(Instruction::Break);
            },
            ast::Statement::Continue => {
                self.emit(Instruction::Continue);
            },
            ast::Statement::Return { value } => {
                match value {
                    Some(e) => {
                        let size = e.len();
                        for v in e {
                            self.compile_expression(v);
                        }

                        // If we have more than 1 return value, make it a tuple:
                        if size > 1 {
                            self.emit(Instruction::BuildTuple { size });
                        }
                    },
                    None => {
                        // TODO: Put none on stack
                    }
                }

                self.emit(Instruction::ReturnValue);
            },
            ast::Statement::Assign { targets, value } => {
                self.compile_expression(value);

                for target in targets {
                    match target {
                        ast::Expression::Identifier { name } => {
                            self.emit(Instruction::StoreName { name: name });
                        },
                        _ => {
                            panic!("WTF");
                        }
                    }
                }
            },
            ast::Statement::Delete { targets } => {
                // Remove the given names from the scope
                // self.emit(Instruction::DeleteName);
            },
            ast::Statement::Pass => {
                self.emit(Instruction::Pass);
            },
        }
    }

    fn compile_expression(&mut self, expression: ast::Expression) {
        trace!("Compiling {:?}", expression);
        match expression {
            ast::Expression::Call { function, args } => {
                self.compile_expression(*function);
                let count = args.len();
                for arg in args {
                    self.compile_expression(arg)
                }
                self.emit(Instruction::CallFunction { count: count });
            },
            ast::Expression::Binop { a, op, b } => {
                self.compile_expression(*a);
                self.compile_expression(*b);

                // Perform operation:
                let i = match op {
                    ast::Operator::Add => Instruction::BinaryAdd,
                    ast::Operator::Sub => Instruction::BinarySubtract,
                    ast::Operator::Mult => Instruction::BinaryMultiply,
                    ast::Operator::MatMult => Instruction::BinaryMatrixMultiply,
                    ast::Operator::Div => Instruction::BinaryDivide,
                    ast::Operator::FloorDiv => Instruction::BinaryFloorDivide,
                    ast::Operator::Mod => Instruction::BinaryModulo,
                    ast::Operator::Pow => Instruction::BinaryPower,
                    ast::Operator::LShift => Instruction::BinaryLshift,
                    ast::Operator::RShift => Instruction::BinaryRshift,
                    ast::Operator::BitOr => Instruction::BinaryOr,
                    ast::Operator::BitXor => Instruction::BinaryXor,
                    ast::Operator::BitAnd => Instruction::BinaryAnd,
                };
                self.emit(i);
            },
            ast::Expression::Number { value } => {
                self.emit(Instruction::LoadConst { value });
            },
            ast::Expression::List { elements } => {
                let size = elements.len();
                for element in elements {
                    self.compile_expression(element);
                }
                self.emit(Instruction::BuildList { size: size });
            },
            ast::Expression::Tuple { elements } => {
                // Load const?
            },
            ast::Expression::True => {
                self.emit(Instruction::LoadConst { value: 1 });
            },
            ast::Expression::False => {
                self.emit(Instruction::LoadConst { value: 0 });
            },
            ast::Expression::None => {
                self.emit(Instruction::LoadConst { value: 0 });
            },
            ast::Expression::String { value } => {
                self.emit(Instruction::LoadStringConstant { value });
            },
            ast::Expression::Identifier { name } => {
                self.emit(Instruction::LoadName { name });
            },
        }
    }

    fn emit(&mut self, instruction: Instruction)
    {
        self.codeobject.instructions.push(instruction);
    }
}
