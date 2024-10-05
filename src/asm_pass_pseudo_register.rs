use crate::asm;
use crate::asm::{Instruction, Operand, Program};
use crate::asm_pass::AsmPass;
use crate::stack_allocator::StackAllocator;

/**
This pass looks for all the pseudo register references in the asm tree and replace them with a stack offset.
*/
pub struct AsmPassPseudoRegister {
    stack_allocator: StackAllocator
}

impl AsmPassPseudoRegister {
    pub fn new() -> Self {
        Self {
            stack_allocator: StackAllocator::new()
        }
    }

    fn handle_function(&mut self, function: asm::Function) -> asm::Function {
        let (name, ins) = function.decompose();
        let mut instructions: Vec<_> = ins.into_iter().map(|instruction| self.handle_instruction(instruction)).collect();

        instructions.insert(0, Instruction::AllocateStack(self.stack_allocator.stack_size()));

        asm::Function::new(name, instructions)
    }

    fn handle_instruction(&mut self, instruction: Instruction) -> Instruction {
        match instruction {
            Instruction::Mov { src, dest } => { Instruction::Mov {
                src: self.handle_operand(src),
                dest : self.handle_operand(dest)
            } },
            Instruction::Unary(operator, operand) => { Instruction::Unary(operator, self.handle_operand(operand)) },
            instruction => instruction
        }
    }

    fn handle_operand(&mut self, operand: Operand) -> Operand {
        match operand {
            Operand::Pseudo(pseudo_register) => { Operand::Stack(self.stack_allocator.get_stack_offset(pseudo_register.as_str())) },
            operand => operand
        }
    }
}

impl AsmPass for AsmPassPseudoRegister {
    fn run(&mut self, program: Program) -> Program {
        let function = self.handle_function(program.into_function());
        Program::from_function(function)
    }
}