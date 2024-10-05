use crate::asm;
use crate::asm::{BinaryOperator, Instruction, Operand, Register, UnaryOperator};
use crate::codewriter::{CodeWriter, LineWriter};

pub fn emit(writer: &mut CodeWriter, program: &asm::Program) {
    emit_function(writer, program.function());
    writer.blank_line();
    //writer.write_line(".section .note.GNU-stack,\"\",@progbits");
}

fn emit_function(writer: &mut CodeWriter, function: &asm::Function) {
    let function_name = if function.name() == "main" {
        "_main"
    } else {
        function.name()
    };

    writer.write_line(format!(".global {}", function_name).as_str());
    writer.write_line(format!("{}:", function_name).as_str());
    writer.write_block(|writer| {
        writer.write_line("pushq %rbp");
        writer.write_line("movq %rsp, %rbp");
        function.instructions().iter().for_each(|inst| {
            emit_instruction(writer, inst);
        })
    })
}

fn emit_instruction(writer: &mut CodeWriter, instruction: &Instruction) {
    match instruction {
        Instruction::Mov { src, dest } => {
            writer.line(|writer| {
                writer.write("movl ");
                emit_operand(writer, src);
                writer.write(", ");
                emit_operand(writer, dest);
            })
        }
        Instruction::Ret => {
            writer.write_line("movq %rbp, %rsp");
            writer.write_line("popq %rbp");
            writer.write_line("ret")
        }
        Instruction::Unary(op, operand) => {
            writer.line(|writer| {
                emit_unary_operator(writer, op);
                writer.write(" ");
                emit_operand(writer, operand);
            })
        }
        Instruction::Cdq => writer.write_line("cdq"),
        Instruction::Idiv(operand) => {
            writer.line(|writer| {
                writer.write("idivl ");
                emit_operand(writer, operand);
            })
        }
        Instruction::Binary(op, left, right) => {
            writer.line(|writer| {
                emit_binary_operator(writer, op);
                writer.write(" ");
                emit_operand(writer, left);
                writer.write(", ");
                emit_operand(writer, right);
            })
        }
        Instruction::AllocateStack(size) => {
            writer.write_line(format!("subq ${}, %rsp", size).as_str());
        }
    }
}

fn emit_unary_operator(writer: &mut LineWriter, operator: &UnaryOperator) {
    match operator {
        UnaryOperator::Neg => writer.write("negl"),
        UnaryOperator::Not => writer.write("notl"),
    }
}

fn emit_binary_operator(writer: &mut LineWriter, operator: &BinaryOperator) {
    match operator {
        BinaryOperator::Add => writer.write("addl"),
        BinaryOperator::Sub => writer.write("subl"),
        BinaryOperator::Mul => writer.write("imull"),
    }
}

fn emit_operand(writer: &mut LineWriter, operand: &Operand) {
    match operand {
        Operand::Register(register) => emit_register(writer, register),
        Operand::Imm(value) => writer.write(format!("${}", value).as_str()),
        Operand::Stack(offset) => writer.write(format!("{}(%rbp)", offset).as_str()),
        Operand::Pseudo(_) => unreachable!("Pseudo registers should have been removed in the PseudoRegister pass"),
    }
}

fn emit_register(writer: &mut LineWriter, register: &Register) {
    match register {
        Register::AX => writer.write("%eax"),
        Register::R10 => writer.write("%r10d"),
        Register::DX => writer.write("%edx"),
        Register::R11 => writer.write("%r11d"),
    }
}