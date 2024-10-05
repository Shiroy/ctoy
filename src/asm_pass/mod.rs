use crate::asm;

mod pseudo_register;
mod invalid_mov_rewrite;

pub use self::pseudo_register::*;
pub use self::invalid_mov_rewrite::*;

pub trait AsmPass {
    fn run(&mut self, program: asm::Program) -> asm::Program;
}