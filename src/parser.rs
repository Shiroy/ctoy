use crate::ast::{BinaryOperator, Expression, Factor, Function, Program, Statement, UnaryOperator};
use crate::lexer::{Token, Tokenizer};
use std::iter::Peekable;

pub type TokenStream<'a> = Peekable<Tokenizer<'a>>;

macro_rules! expect_token {
    ($tokenizer: ident, $token: expr) => {{
        let token = next_token($tokenizer)?;
        if $token != token {
            return Err(format!("Expected token {:?}, got {:?}", $token, token));
        }
        token
    }};
}

fn next_token(tokens: &mut TokenStream) -> Result<Token, String> {
    tokens.next().unwrap_or(Err("Unexpected end-of-file".to_owned()))
}

fn peek(tokens: &mut TokenStream) -> Result<Token, String> {
    match tokens.peek() {
        None => Err("Unexpected end-of-file".to_owned()),
        Some(Err(err)) => Err(err.to_string()),
        Some(Ok(token)) => Ok(token.clone())
    }
}

type ParserResult<T> = Result<T, String>;

pub fn parse(tokens: &mut TokenStream) -> ParserResult<Program> {
    let function = parse_function(tokens)?;

    if tokens.count() > 0 {
        return Err("Extra token in the source".into());
    }

    Ok(Program::new(function))
}

fn parse_function(tokens: &mut TokenStream) -> ParserResult<Function> {
    if Token::KwInt != next_token(tokens)? {
        return Err("Expected token 'int'".to_owned());
    }

    let name = if let Token::Identifier(name) = next_token(tokens)? {
        name
    } else {
        return Err("Expected identifier".to_owned())
    };

    expect_token!(tokens, Token::OpenParenthesis);
    expect_token!(tokens, Token::KwVoid);
    expect_token!(tokens, Token::CloseParenthesis);
    expect_token!(tokens, Token::OpeningBrace);

    let body = parse_statement(tokens)?;

    expect_token!(tokens, Token::ClosingBrace);

    Ok(Function::new(name, body))
}

fn parse_statement(tokens: &mut TokenStream) -> ParserResult<Statement> {
    let statement = match next_token(tokens)? {
        Token::KwReturn => {
            let expression = parse_expression(tokens, 0)?;
            Statement::Return { expr: expression }
        }
        token => { return Err(format!("Unexpected token {:?}", token)); }
    };

    expect_token!(tokens, Token::Semicolon);

    Ok(statement)
}

fn parse_factor(tokens: &mut TokenStream) -> ParserResult<Factor> {
    match peek(tokens)? {
        Token::Constant(_) => {
            let token = next_token(tokens)?;
            if let Token::Constant(value) = token {
                Ok(Factor::Constant(value))
            } else {
                unreachable!()
            }
        }
        Token::Tilde | Token::Hyphen => {
            let operator = parse_unary_operator(tokens)?;
            let expression = parse_factor(tokens)?;

            Ok(Factor::Unary(operator, Box::new(expression)))
        }
        Token::OpenParenthesis => {
            expect_token!(tokens, Token::OpenParenthesis);
            let expression = parse_expression(tokens, 0)?;
            expect_token!(tokens, Token::CloseParenthesis);

            Ok(Factor::Expression(Box::new(expression)))
        }
        token => Err(format!("Unexpected token {:?}", token))
    }
}

fn parse_expression(token: &mut TokenStream, min_precedence: usize) -> ParserResult<Expression> {
    let mut left = Expression::Factor(parse_factor(token)?);

    while {
        let ope = peek_is_binary_operator(token)?;
        ope.is_some() && binary_operator_precedence(ope.as_ref().unwrap()) >= min_precedence
    } {
        let operator = parse_binary_operator(token)?;
        let right = parse_expression(token, binary_operator_precedence(&operator) + 1)?;
        left = Expression::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        };
    }

    Ok(left)
}

fn peek_is_binary_operator(tokens: &mut TokenStream) -> Result<Option<BinaryOperator>, String> {
    let token = peek(tokens)?;
    match token {
        Token::Plus => Ok(Some(BinaryOperator::Add)),
        Token::Hyphen => Ok(Some(BinaryOperator::Sub)),
        Token::Asterisk => Ok(Some(BinaryOperator::Mul)),
        Token::ForwardSlash => Ok(Some(BinaryOperator::Div)),
        Token::Percent => Ok(Some(BinaryOperator::Rem)),
        _ => Ok(None)
    }
}

fn parse_binary_operator(tokens: &mut TokenStream) -> ParserResult<BinaryOperator> {
    match next_token(tokens)? {
        Token::Plus => Ok(BinaryOperator::Add),
        Token::Hyphen => Ok(BinaryOperator::Sub),
        Token::Asterisk => Ok(BinaryOperator::Mul),
        Token::ForwardSlash => Ok(BinaryOperator::Div),
        Token::Percent => Ok(BinaryOperator::Rem),
        _ => Err("Expected binary operator".to_owned())
    }
}

fn parse_unary_operator(tokens: &mut TokenStream) -> ParserResult<UnaryOperator> {
    match next_token(tokens)? {
        Token::Hyphen => Ok(UnaryOperator::Negate),
        Token::Tilde => Ok(UnaryOperator::Complement),
        token => Err(format!("Expected unary operator ('~' or '-'), got {:?}", token))
    }
}

fn binary_operator_precedence(binary_operator: &BinaryOperator) -> usize {
    match binary_operator {
        BinaryOperator::Div => 50,
        BinaryOperator::Add => 45,
        BinaryOperator::Mul => 50,
        BinaryOperator::Sub => 45,
        BinaryOperator::Rem => 50
    }
}