mod lexer;
mod ast;
mod parser;

use std::fs::File;
use std::io::Read;
use clap::Parser;
use crate::lexer::{Tokenizer};
use crate::parser::parse;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse:bool,

    #[arg(long)]
    codegen: bool,

    program: String
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let source = {
        let mut f = File::open(cli.program)?;
        let mut source = String::new();
        f.read_to_string(&mut source)?;
        source
    };

    let mut tokenizer = Tokenizer::new(source.as_str());
    let ast = parse(&mut tokenizer).unwrap();
    
    println!("{:?}", ast);

    Ok(())
}
