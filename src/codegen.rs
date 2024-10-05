use crate::asm;
use crate::asm::Instruction::Binary;
use crate::asm::{Operand, Register};
use crate::tacky;

pub fn codegen(program: &tacky::Program) -> asm::Program {
    asm::Program::new(
        codegen_function(program.function())
    )
}

fn codegen_function(function: &tacky::Function) -> asm::Function {
    let mut instruction = vec![];
    for instr in function.instructions() {
        codegen_instruction(instr, &mut instruction);
    }

    asm::Function::new(function.identifier().to_owned(), instruction)
}

fn codegen_instruction(instr: &tacky::Instruction, instructions: &mut Vec<asm::Instruction>) {
    match instr {
        tacky::Instruction::Return { val } => codegen_ret(instructions, val),
        tacky::Instruction::Unary { operator, src, dst } => codegen_unary(instructions, operator, src, dst),
        tacky::Instruction::Binary { operator, lhs, rhs, dst } => codegen_binary(instructions, operator, lhs, rhs, dst)
    }
}

fn codegen_ret(instructions: &mut Vec<asm::Instruction>, val: &tacky::Value) {
    let src_operand = codegen_operand(val);

    instructions.push(
        asm::Instruction::Mov {
            src: src_operand,
            dest: asm::Operand::Register(asm::Register::AX),
        }
    );

    instructions.push(asm::Instruction::Ret);
}

fn codegen_operand(operand: &tacky::Value) -> asm::Operand {
    match operand {
        tacky::Value::Var { identifier } => asm::Operand::Pseudo(identifier.clone()),
        tacky::Value::Constant(value) => asm::Operand::Imm(value.clone()),
    }
}

fn codegen_unary(instructions: &mut Vec<asm::Instruction>, operator: &tacky::UnaryOperator, src: &tacky::Value, dst: &tacky::Value) {
    let src_operand = codegen_operand(src);
    let dst_operand = codegen_operand(dst);
    let op = codegen_unary_op(operator);

    if let asm::Operand::Imm(_) = dst_operand {
        unreachable!("Destination operand cannot be a value");
    }

    instructions.push(
        asm::Instruction::Mov {
            src: src_operand,
            dest: dst_operand.clone(),
        }
    );

    instructions.push(
        asm::Instruction::Unary(op, dst_operand)
    );
}

fn codegen_unary_op(unary_op: &tacky::UnaryOperator) -> asm::UnaryOperator {
    match unary_op {
        tacky::UnaryOperator::Negate => asm::UnaryOperator::Neg,
        tacky::UnaryOperator::Complement => asm::UnaryOperator::Not
    }
}

fn codegen_binary(instructions: &mut Vec<asm::Instruction>, operator: &tacky::BinaryOperator, left: &tacky::Value, right: &tacky::Value, dst: &tacky::Value) {
    let left_operand = codegen_operand(left);
    let right_operand = codegen_operand(right);
    let dst_operand = codegen_operand(dst);

    match operator {
        tacky::BinaryOperator::Add
        | tacky::BinaryOperator::Subtract
        | tacky::BinaryOperator::Multiply => {
            let op = codegen_binary_op(operator);

            instructions.push(
                asm::Instruction::Mov {
                    src: left_operand,
                    dest: dst_operand.clone(),
                }
            );

            instructions.push(
                Binary(op, right_operand, dst_operand)
            )
        }

        tacky::BinaryOperator::Divide
        | tacky::BinaryOperator::Remainder => {
            instructions.push(asm::Instruction::Mov {
                src: left_operand,
                dest: asm::Operand::Register(Register::AX),
            });

            instructions.push(
                asm::Instruction::Cdq
            );

            instructions.push(
                asm::Instruction::Idiv(right_operand)
            );

            if *operator == tacky::BinaryOperator::Divide {
                instructions.push(
                    asm::Instruction::Mov {
                        src: Operand::Register(Register::AX),
                        dest: dst_operand,
                    }
                );
            } else {
                instructions.push(
                    asm::Instruction::Mov {
                        src: Operand::Register(Register::DX),
                        dest: dst_operand,
                    }
                );
            }
        }
    }
}

fn codegen_binary_op(binary_op: &tacky::BinaryOperator) -> asm::BinaryOperator {
    match binary_op {
        tacky::BinaryOperator::Add => asm::BinaryOperator::Add,
        tacky::BinaryOperator::Subtract => asm::BinaryOperator::Sub,
        tacky::BinaryOperator::Multiply => asm::BinaryOperator::Mul,
        _ => { unreachable!("Division and remainder are handled separately"); }
    }
}