use crate::asm;
use crate::asm::{Instruction, Operand, Program, Register};
use crate::asm_pass::AsmPass;

pub struct InvalidMovRewrite {}

impl InvalidMovRewrite {
    pub fn new() -> Self { Self {} }

    fn handle_function(&mut self, function: asm::Function) -> asm::Function {
        let (name, ins) = function.decompose();
        let instructions: Vec<_> = ins.into_iter().map(|instruction| self.handle_instruction(instruction)).flatten().collect();

        asm::Function::new(name, instructions)
    }

    fn handle_instruction(&mut self, instruction: Instruction) -> Vec<Instruction> {
        match instruction {
            Instruction::Mov { src, dest } => {
                match (src, dest) {
                    (Operand::Stack(offset_src), Operand::Stack(offset_dst)) => {
                        vec![
                            Instruction::Mov {
                                src: Operand::Stack(offset_src),
                                dest: Operand::Register(Register::R10)
                            },
                            Instruction::Mov {
                                src: Operand::Register(Register::R10),
                                dest: Operand::Stack(offset_dst),
                            }
                        ]
                    }
                    (src, dest) => vec![Instruction::Mov { src, dest }]
                }
            }
            instruction => vec![instruction]
        }
    }
}

impl AsmPass for InvalidMovRewrite {
    fn run(&mut self, program: Program) -> Program {
        let function = self.handle_function(program.into_function());
        Program::from_function(function)
    }
}