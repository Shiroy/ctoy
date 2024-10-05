use crate::tacky;
use crate::asm;
use crate::asm::{Instruction, Operand};

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
        tacky::Instruction::Unary { operator, src, dst } => codegen_unary(instructions, operator, src, dst)
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

// fn replace_pseudo_register(program: &mut asm::Program) {
//     replace_pseudo_register_function(&mut program.function());
// }
//
// fn replace_pseudo_register_function(function: &mut asm::Function) {
//
// }
//
// fn replace_pseudo_register_instruction(instruction: &mut asm::Instruction) {
//     match instruction {
//         Instruction::Mov { src, dest } => {}
//         Instruction::Unary(_, _) => {}
//         Instruction::AllocateStack(_) => {}
//         Instruction::Ret => {}
//     }
// }
//
// fn replace_pseudo_register_operand(operand: &Operand) {
//     match operand {
//         Operand::Pseudo(identifier) => { Operand::Stack(4) },
//         rem =>
//     }
// }