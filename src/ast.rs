#[derive(Debug)]
pub(crate) struct Program {
    function: Box<Function>
}

impl Program {
    pub fn new(function: Function) -> Self {
        Program {
            function: Box::new(function)
        }
    }
}

#[derive(Debug)]
pub(crate) struct Function {
    name: String,
    body: Box<Statement>
}

impl Function {
    pub(crate) fn new(name: String, body: Statement) -> Function {
        Function {
            name,
            body: Box::new(body)
        }
    }
}

#[derive(Debug)]
pub(crate) enum Statement {
    Return { expr: Box<Expression> }
}

#[derive(Debug)]
pub(crate) enum Expression {
    Constant(u64)
}