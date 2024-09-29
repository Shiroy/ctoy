#[derive(Debug)]
pub struct Program {
    function: Function,
}

impl Program {
    pub fn new(function: Function) -> Self {
        Program {
            function
        }
    }

    pub fn function(&self) -> &Function {
        &self.function
    }
}

#[derive(Debug)]
pub struct Function {
    name: String,
    instructions: Vec<Instruction>,
}

impl Function {
    pub fn new(name: String, instructions: Vec<Instruction>) -> Self {
        Function {
            name,
            instructions,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dest: Operand },
    Ret,
}

#[derive(Debug)]
pub enum Operand {
    Imm(u64),
    Register,
}

