use crate::ast::{Expression, Function, Program, Statement};
use crate::CompilerError::ParserError;
use crate::lexer::{Token, Tokenizer};

macro_rules! expect_token {
    ($tokenizer: ident, $token: expr) => {{
        let token = next_token($tokenizer)?;
        if $token != token {
            return Err(format!("Expected token {:?}, got {:?}", $token, token));
        }
    }};
}

fn next_token(tokens: &mut Tokenizer) -> Result<Token, String> {
    tokens.next().unwrap_or(Err("Unexpected end-of-file".to_owned()))
}

type ParserResult<T> = Result<T, String>;

pub fn parse(tokens: &mut Tokenizer) -> ParserResult<Program> {
    let function = parse_function(tokens)?;

    if tokens.count() > 0 {
        return Err("Extra token in the source".into());
    }

    Ok(Program::new(function))
}

fn parse_function(tokens: &mut Tokenizer) -> ParserResult<Function> {
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

fn parse_statement(tokens: &mut Tokenizer) -> ParserResult<Statement> {
    let statement = match next_token(tokens)? {
        Token::KwReturn => {
            let expression = parse_expression(tokens)?;
            Statement::Return { expr: expression }
        }
        token => { return Err(format!("Unexpected token {:?}", token)); }
    };

    expect_token!(tokens, Token::Semicolon);

    Ok(statement)
}

fn parse_expression(tokens: &mut Tokenizer) -> ParserResult<Expression> {
    let token = next_token(tokens)?;
    match token {
        Token::Constant(value) => Ok(Expression::Constant(value)),
        _ => Err(format!("Unexpected token {:?}", token))
    }
}