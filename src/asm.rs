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

    pub fn from_function(function: Function) -> Self {
        Self {
            function
        }
    }

    pub fn function(&self) -> &Function {
        &self.function
    }

    pub fn into_function(self) -> Function {
        self.function
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

    pub fn decompose(self) -> (String, Vec<Instruction>) {
        (self.name, self.instructions)
    }
}

#[derive(Debug)]
pub enum Instruction {
    Mov { src: Operand, dest: Operand },
    Unary(UnaryOperator, Operand),
    AllocateStack(i64),
    Ret,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Neg,
    Not,
}


#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Operand {
    Imm(u64),
    Register(Register),
    Pseudo(String),
    Stack(i64),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Register {
    AX,
    R10,
}

