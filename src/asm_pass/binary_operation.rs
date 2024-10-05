use crate::asm;
use crate::asm::{BinaryOperator, Instruction, Operand, Program, Register};
use crate::asm_pass::AsmPass;

pub struct BinaryOperation {}

impl BinaryOperation {
    pub fn new() -> Self {
        Self {}
    }

    fn handle_function(&mut self, function: asm::Function) -> asm::Function {
        let (name, ins) = function.decompose();
        let instructions: Vec<_> = ins.into_iter().map(|instruction| self.handle_instruction(instruction)).flatten().collect();

        asm::Function::new(name, instructions)
    }

    fn handle_instruction(&mut self, instruction: Instruction) -> Vec<Instruction> {
        match instruction {
            Instruction::Idiv(operand) => {
                if let Operand::Imm(value) = operand {
                    vec![
                        Instruction::Mov {
                            src: Operand::Imm(value),
                            dest: Operand::Register(Register::R10)
                        },
                        Instruction::Idiv(
                            Operand::Register(Register::R10)
                        )
                    ]
                } else {
                    vec![Instruction::Idiv(operand)]
                }
            }
            Instruction::Binary(operator, src, dest) if operator == BinaryOperator::Add || operator == BinaryOperator::Sub => {
                match (src, dest) {
                    (Operand::Stack(src_offset), Operand::Stack(dest_offset)) => {
                        vec![
                            Instruction::Mov {
                                src: Operand::Stack(src_offset),
                                dest: Operand::Register(Register::R10)
                            },
                            Instruction::Binary(
                                operator,
                                Operand::Register(Register::R10),
                                Operand::Stack(dest_offset)
                            )
                        ]
                    }
                    (src, dest) => vec![Instruction::Binary(operator, src, dest)]
                }
            }
            Instruction::Binary(operator, src, dest) if operator == BinaryOperator::Mul => {
                if let Operand::Stack(dest_offset) = dest {
                    vec![
                        Instruction::Mov {
                            src: Operand::Stack(dest_offset),
                            dest: Operand::Register(Register::R11)
                        },
                        Instruction::Binary(operator, src, Operand::Register(Register::R11)),
                        Instruction::Mov {
                            src: Operand::Register(Register::R11),
                            dest: Operand::Stack(dest_offset)
                        }
                    ]
                } else {
                    vec![Instruction::Binary(operator, src, dest)]
                }
            }
            instruction => vec![instruction]
        }
    }
}

impl AsmPass for BinaryOperation {
    fn run(&mut self, program: Program) -> Program {
        let function = self.handle_function(program.into_function());
        Program::from_function(function)
    }
}