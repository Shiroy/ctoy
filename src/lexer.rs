use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    OpenParenthesis,
    CloseParenthesis,
    OpeningBrace,
    ClosingBrace,
    Constant(u64),
    Semicolon,
    KwInt,
    KwReturn,
    KwVoid,
    Hyphen,
    TwoHyphens,
    Tilde,

    Plus,
    Asterisk,
    ForwardSlash,
    Percent,
}

pub struct Tokenizer<'a> {
    identifier_regex: Regex,
    constant_regex: Regex,
    kw_int_regex: Regex,
    kw_void_regex: Regex,
    kw_return_regex: Regex,
    open_parenthesis_regex: Regex,
    close_parenthesis_regex: Regex,
    open_brace_regex: Regex,
    close_brace_regex: Regex,
    semicolon_regex: Regex,
    two_hyphens_regex: Regex,
    hyphen_regex: Regex,
    tilde_regex: Regex,
    plus_regex: Regex,
    asterisk_regex: Regex,
    forward_slash_regex: Regex,
    percent_regex: Regex,

    input: &'a str,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &'a str) -> Self {
        Tokenizer {
            identifier_regex: Regex::new(r"^[a-zA-Z_]\w*\b").unwrap(),
            constant_regex: Regex::new(r"^[0-9]+\b").unwrap(),
            kw_int_regex: Regex::new(r"^int\b").unwrap(),
            kw_void_regex: Regex::new(r"^void\b").unwrap(),
            kw_return_regex: Regex::new(r"^return\b").unwrap(),
            open_parenthesis_regex: Regex::new(r"^\(").unwrap(),
            close_parenthesis_regex: Regex::new(r"^\)").unwrap(),
            open_brace_regex: Regex::new(r"^\{").unwrap(),
            close_brace_regex: Regex::new(r"^}").unwrap(),
            semicolon_regex: Regex::new(r"^;").unwrap(),
            hyphen_regex: Regex::new(r"^-").unwrap(),
            two_hyphens_regex: Regex::new(r"^--").unwrap(),
            tilde_regex: Regex::new(r"^~").unwrap(),
            plus_regex: Regex::new(r"^\+").unwrap(),
            asterisk_regex: Regex::new(r"^\*").unwrap(),
            forward_slash_regex: Regex::new(r"^/").unwrap(),
            percent_regex: Regex::new(r"^%").unwrap(),

            input: &source,
        }
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let input = self.input.trim_start();
            if input.len() == 0 {
                return None;
            }

            if let Some(result) = self.identifier_regex.find(input) {
                let identifier = result.as_str();
                let (_, next) = input.split_at(result.len());
                self.input = next;

                let token = if self.kw_int_regex.is_match(identifier) {
                    Token::KwInt
                } else if self.kw_return_regex.is_match(identifier) {
                    Token::KwReturn
                } else if self.kw_void_regex.is_match(identifier) {
                    Token::KwVoid
                } else {
                    Token::Identifier(identifier.to_owned())
                };

                return Some(Ok(token));
            } else if let Some(result) = self.constant_regex.find(input) {
                let value = result.as_str();
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::Constant(u64::from_str(value).unwrap())));
            } else if let Some(result) = self.open_brace_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::OpeningBrace));
            } else if let Some(result) = self.close_brace_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::ClosingBrace));
            } else if let Some(result) = self.open_parenthesis_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::OpenParenthesis));
            } else if let Some(result) = self.semicolon_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::Semicolon));
            } else if let Some(result) = self.close_parenthesis_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::CloseParenthesis));
            } else if let Some(result) = self.two_hyphens_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::TwoHyphens));
            } else if let Some(result) = self.hyphen_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::Hyphen));
            } else if let Some(result) = self.tilde_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::Tilde));
            } else if let Some(result) = self.plus_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;

                return Some(Ok(Token::Plus));
            } else if let Some(result) = self.asterisk_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;
                return Some(Ok(Token::Asterisk));
            } else if let Some(result) = self.forward_slash_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;
                return Some(Ok(Token::ForwardSlash));
            } else if let Some(result) = self.percent_regex.find(input) {
                let (_, next) = input.split_at(result.len());
                self.input = next;
                return Some(Ok(Token::Percent));
            } else {
                return Some(Err("Unknown token".to_owned()))
            }
        }
    }
}