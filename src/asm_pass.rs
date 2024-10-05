use crate::asm;

pub trait AsmPass {
    fn run(&mut self, program: asm::Program) -> asm::Program;
}