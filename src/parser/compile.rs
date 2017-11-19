
/*
 * Take an AST and transform it into bytecode
 */

use parser::ast;
use parser::bytecode;

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
        println!("Compiling {:?}", statement);
      match statement {
        ast::Statement::Expression { expression } => {
          self.compile_expression(expression);
          self.emit(bytecode::Instruction::Pop);
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
        println!("Compiling {:?}", expression);
      match expression {
        ast::Expression::Call { f, args } => {
          // compiler.bytecode.add(0x1)
          self.compile_expression(*f);
          for arg in args {
              self.compile_expression(arg)
          }
          self.emit(bytecode::Instruction::CallFunction);
        },
        ast::Expression::Binop { a, op, b } => {
          self.compile_expression(*a);
          self.compile_expression(*b);
          println!("{}", op);
        },
        ast::Expression::Number { value } => {
          // compiler.bytecode.add(0x1)
          self.emit(bytecode::Instruction::LoadConst { value });
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
