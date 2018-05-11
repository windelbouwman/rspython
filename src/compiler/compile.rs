
/*
 * Take an AST and transform it into bytecode
 */

use super::ast;
use super::bytecode;

struct Compiler {
  codeobject: bytecode::CodeObject,
}

pub fn compile(p: ast::Program) -> bytecode::CodeObject {
    let mut compiler = Compiler {
        codeobject: bytecode::CodeObject::new(),
    };

    for statement in p.statements {
        compiler.compile_statement(statement);
    }

    compiler.codeobject
}

impl Compiler {
    fn compile_statement(&mut self, statement: ast::Statement) {
        trace!("Compiling {:?}", statement);
        match statement {
            ast::Statement::Expression { expression } => {
                self.compile_expression(expression);
                self.emit(bytecode::Instruction::Pop);
            },
            ast::Statement::If { test, body } => {
                self.compile_expression(test);
                for inner_statement in body {
                    self.compile_statement(inner_statement)
                }
            },
            ast::Statement::While { test, body } => {
                self.compile_expression(test);
                for inner_statement in body {
                    self.compile_statement(inner_statement)
                }
            },
            ast::Statement::With { items, body } => {
                // TODO
            },
            ast::Statement::For { test } => {},
            ast::Statement::FunctionDef { name, body } => {
                for inner_statement in body {
                    self.compile_statement(inner_statement)
                }
            },
            ast::Statement::ClassDef { name } => {
                // TODO?
            },
            ast::Statement::Assert { test, msg } => {
                // TODO?
            },
            ast::Statement::Break => {
                self.emit(bytecode::Instruction::Break);
            },
            ast::Statement::Continue => {
                self.emit(bytecode::Instruction::Continue);
            },
            ast::Statement::Pass => {
                self.emit(bytecode::Instruction::Pass);
            },
        }
    }

    fn compile_expression(&mut self, expression: ast::Expression) {
        trace!("Compiling {:?}", expression);
        match expression {
            ast::Expression::Call { function, args } => {
                // compiler.bytecode.add(0x1)
                self.compile_expression(*function);
                let count = args.len();
                for arg in args {
                    self.compile_expression(arg)
                }
                self.emit(bytecode::Instruction::CallFunction { count: count });
            },
            ast::Expression::Binop { a, op, b } => {
                self.compile_expression(*a);
                self.compile_expression(*b);

                // Perform operation:
                match op {
                    ast::Operator::Sub => {
                        self.emit(bytecode::Instruction::BinarySubtract);
                    },
                    ast::Operator::Add => {
                        self.emit(bytecode::Instruction::BinaryAdd);
                    },
                    _ => {
                        panic!("NOTIMPL");
                    }
                }
            },
            ast::Expression::Number { value } => {
                self.emit(bytecode::Instruction::LoadConst { value });
            },
            ast::Expression::True => {
                self.emit(bytecode::Instruction::LoadConst { value: 1 });
            },
            ast::Expression::False => {
                self.emit(bytecode::Instruction::LoadConst { value: 0 });
            },
            ast::Expression::None => {
                self.emit(bytecode::Instruction::LoadConst { value: 0 });
            },
            ast::Expression::String { value } => {
                self.emit(bytecode::Instruction::LoadStringConstant { value });
            },
            ast::Expression::Identifier { name } => {
                self.emit(bytecode::Instruction::LoadName { name });
            },
        }
    }

    fn emit(&mut self, instruction: bytecode::Instruction)
    {
        self.codeobject.instructions.push(instruction);
    }
}
