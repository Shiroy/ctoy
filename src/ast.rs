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
pub(crate) struct Function {
    name: String,
    body: Statement,
}

impl Function {
    pub(crate) fn new(name: String, body: Statement) -> Function {
        Function {
            name,
            body,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn body(&self) -> &Statement {
        &self.body
    }
}

#[derive(Debug)]
pub(crate) enum Statement {
    Return { expr: Expression }
}

#[derive(Debug)]
pub(crate) enum Expression {
    Factor(Factor),
    Binary { left: Box<Expression>, operator: BinaryOperator, right: Box<Expression> },
}

#[derive(Debug)]
pub(crate) enum Factor {
    Constant(u64),
    Unary(UnaryOperator, Box<Factor>),
    Expression(Box<Expression>),
}

#[derive(Debug)]
pub(crate) enum UnaryOperator {
    Complement,
    Negate,
}

#[derive(Debug)]
pub(crate) enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}