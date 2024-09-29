use crate::ast;
use crate::asm;

pub fn codegen(ast: &ast::Program) -> asm::Program {
    asm::Program::new(
        codegen_function(ast.function())
    )
}

fn codegen_function(function: &ast::Function) -> asm::Function {
    let instructions = codegen_statement(function.body());

    asm::Function::new(function.name().to_owned(), instructions)
}

fn codegen_statement(statement: &ast::Statement) -> Vec<asm::Instruction> {
    match statement {
        ast::Statement::Return { expr } => {
            let operand = codegen_expression(expr);
            let instructions = vec![
                asm::Instruction::Mov { src: operand, dest: asm::Operand::Register },
                asm::Instruction::Ret
            ];

            instructions
        }
    }
}

fn codegen_expression(expr: &ast::Expression) -> asm::Operand {
    match expr {
        ast::Expression::Constant(value) => asm::Operand::Imm(value.clone()),
        ast::Expression::Unary(_, _) => { todo!() }
    }
}