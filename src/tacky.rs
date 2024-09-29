use crate::ast;
use crate::tacky::Instruction::Return;

#[derive(Debug)]
pub struct Program {
    function: Function,
}

#[derive(Debug)]
pub struct Function {
    identifier: String,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
pub enum Instruction {
    Return { val: Value },
    Unary { operator: UnaryOperator, src: Value, dst: Value },
}

#[derive(Clone, Debug)]
pub enum Value {
    Constant(u64),
    Var { identifier: String },
}

#[derive(Debug)]
pub enum UnaryOperator {
    Complement,
    Negate,
}

struct VariableNameGenerator {
    counter: usize,
}

impl VariableNameGenerator {
    pub fn new() -> Self {
        VariableNameGenerator {
            counter: 0
        }
    }

    pub fn make_temporary(&mut self) -> String {
        let var_name = format!("tmp.{}", self.counter);
        self.counter += 1;
        var_name
    }
}

pub struct TackEmitter {
    variable_name_generator: VariableNameGenerator,
}

impl TackEmitter {
    pub fn new() -> Self {
        TackEmitter {
            variable_name_generator: VariableNameGenerator::new()
        }
    }

    pub fn emit_program(&mut self, program: &ast::Program) -> Program {
        Program {
            function: self.emit_function(program.function())
        }
    }
    fn emit_function(&mut self, function: &ast::Function) -> Function {
        let mut instructions = vec![];

        self.emit_statement(function.body(), &mut instructions);

        Function {
            identifier: function.name().to_owned(),
            instructions,
        }
    }

    fn emit_statement(&mut self, statement: &ast::Statement, instructions: &mut Vec<Instruction>) {
        match statement {
            ast::Statement::Return { expr } => {
                let return_val = self.emit_expression(expr, instructions);
                instructions.push(Return {
                    val: return_val
                });
            }
        }
    }

    fn emit_expression(&mut self, expr: &ast::Expression, instructions: &mut Vec<Instruction>) -> Value {
        match expr {
            ast::Expression::Constant(value) => Value::Constant(value.clone()),
            ast::Expression::Unary(op, expr) => {
                let src = self.emit_expression(expr, instructions);
                let dst = Value::Var { identifier: self.variable_name_generator.make_temporary() };
                let operator = self.emit_unary_operator(op);

                instructions.push(Instruction::Unary {
                    operator,
                    src,
                    dst: dst.clone(),
                });

                dst
            }
        }
    }

    fn emit_unary_operator(&mut self, operator: &ast::UnaryOperator) -> UnaryOperator {
        match operator {
            ast::UnaryOperator::Complement => UnaryOperator::Complement,
            ast::UnaryOperator::Negate => UnaryOperator::Negate
        }
    }
}
