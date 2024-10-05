use crate::ast;
use crate::tacky::Instruction::Return;

#[derive(Debug)]
pub struct Program {
    function: Function,
}

impl Program {
    pub fn function(&self) -> &Function {
        &self.function
    }
}

#[derive(Debug)]
pub struct Function {
    identifier: String,
    instructions: Vec<Instruction>,
}

impl Function {
    pub fn identifier(&self) -> &str {
        &self.identifier
    }

    pub fn instructions(&self) -> &Vec<Instruction> {
        &self.instructions
    }
}

#[derive(Debug)]
pub enum Instruction {
    Return { val: Value },
    Unary { operator: UnaryOperator, src: Value, dst: Value },
    Binary { operator: BinaryOperator, lhs: Value, rhs: Value, dst: Value },
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

#[derive(Debug, Eq, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
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

    fn emit_expression(&mut self, expression: &ast::Expression, instructions: &mut Vec<Instruction>) -> Value {
        match expression {
            ast::Expression::Factor(factor) => {
                self.emit_factor(factor, instructions)
            }
            ast::Expression::Binary { left, right, operator } => {
                let left_result = self.emit_expression(left, instructions);
                let right_result = self.emit_expression(right, instructions);
                let operator = self.emit_binary_operator(operator);

                let result = Value::Var { identifier: self.variable_name_generator.make_temporary() };

                instructions.push(Instruction::Binary {
                    lhs: left_result,
                    rhs: right_result,
                    operator,
                    dst: result.clone(),
                });

                result
            }
        }
    }

    fn emit_factor(&mut self, factor: &ast::Factor, instructions: &mut Vec<Instruction>) -> Value {
        match factor {
            ast::Factor::Constant(value) => Value::Constant(value.clone()),
            ast::Factor::Unary(op, unary_factor) => {
                let src = self.emit_factor(unary_factor, instructions);
                let dst = Value::Var { identifier: self.variable_name_generator.make_temporary() };
                let operator = self.emit_unary_operator(op);

                instructions.push(Instruction::Unary {
                    operator,
                    src,
                    dst: dst.clone(),
                });

                dst
            }
            ast::Factor::Expression(expr) => {
                self.emit_expression(expr, instructions)
            }
        }
    }

    fn emit_unary_operator(&mut self, operator: &ast::UnaryOperator) -> UnaryOperator {
        match operator {
            ast::UnaryOperator::Complement => UnaryOperator::Complement,
            ast::UnaryOperator::Negate => UnaryOperator::Negate
        }
    }

    fn emit_binary_operator(&self, operator: &ast::BinaryOperator) -> BinaryOperator {
        match operator {
            ast::BinaryOperator::Add => BinaryOperator::Add,
            ast::BinaryOperator::Sub => BinaryOperator::Subtract,
            ast::BinaryOperator::Mul => BinaryOperator::Multiply,
            ast::BinaryOperator::Div => BinaryOperator::Divide,
            ast::BinaryOperator::Rem => BinaryOperator::Remainder,
        }
    }
}
