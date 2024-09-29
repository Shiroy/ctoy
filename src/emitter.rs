use crate::asm;
use crate::asm::{Instruction, Operand};
use crate::codewriter::{CodeWriter, LineWriter};

pub fn emit(writer: &mut CodeWriter, program: &asm::Program) {
    emit_function(writer, program.function());
    writer.blank_line();
    writer.write_line(".section .note.GNU-stack,\"\",@progbits");
}

fn emit_function(writer: &mut CodeWriter, function: &asm::Function) {
    writer.write_line(format!(".global {}", function.name()).as_str());
    writer.write_line(format!("{}:", function.name()).as_str());
    writer.write_block(|writer| {
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
            writer.write_line("ret")
        }
    }
}

fn emit_operand(writer: &mut LineWriter, operand: &Operand) {
    match operand {
        Operand::Register => writer.write("%eax"),
        Operand::Imm(value) => writer.write(format!("${}", value).as_str())
    }
}