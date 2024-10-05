mod lexer;
mod ast;
mod parser;
mod asm;
mod codegen;
mod codewriter;
mod emitter;
mod tacky;
mod stack_allocator;
mod asm_pass;
mod asm_pass_pseudo_register;

use std::fs;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::{PathBuf};
use std::process::{ExitCode, Termination};
use clap::Parser;
use crate::asm_pass::AsmPass;
use crate::asm_pass_pseudo_register::AsmPassPseudoRegister;
use crate::codegen::codegen;
use crate::codewriter::CodeWriter;
//use crate::emitter::emit;
use crate::lexer::{Tokenizer};
use crate::parser::parse;
use crate::tacky::TackEmitter;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    lex: bool,

    #[arg(long)]
    parse: bool,

    #[arg(long)]
    codegen: bool,

    #[arg(long)]
    tacky: bool,

    program: String,
}

#[derive(Debug)]
enum CompilerError {
    LexerError(String),
    ParserError(String),
    IoError(std::io::Error),
}

impl Termination for CompilerError {
    fn report(self) -> ExitCode {
        match self {
            CompilerError::LexerError(_) => ExitCode::FAILURE,
            CompilerError::ParserError(_) => ExitCode::FAILURE,
            CompilerError::IoError(_) => ExitCode::FAILURE
        }
    }
}

impl From<Error> for CompilerError {
    fn from(value: Error) -> Self {
        CompilerError::IoError(value)
    }
}

struct FileSet {
    source: PathBuf,
    preprocessed_source: PathBuf,
    assembly_file: PathBuf,
    executable: PathBuf,
}

impl FileSet {
    pub fn from_source_path(source: &str) -> FileSet {
        let mut assembly_file = PathBuf::from(&source);
        assembly_file.set_extension("s");

        let mut preprocessed_source = PathBuf::from(&source);
        preprocessed_source.set_extension("pre.c");

        let mut executable = PathBuf::from(&source);
        executable.set_extension("");

        FileSet {
            source: PathBuf::from(source),
            assembly_file,
            executable,
            preprocessed_source,
        }
    }

    pub fn source(&self) -> &PathBuf {
        &self.source
    }

    pub fn preprocessed_source(&self) -> &PathBuf {
        &self.preprocessed_source
    }

    pub fn assembly_file(&self) -> &PathBuf {
        &self.assembly_file
    }

    pub fn executable(&self) -> &PathBuf {
        &self.executable
    }
}

fn main() -> Result<(), CompilerError> {
    let cli = Cli::parse();

    let file_set = FileSet::from_source_path(cli.program.as_str());

    std::process::Command::new("gcc").args(["-E",
        "-P",
        file_set.source().to_str().unwrap(),
        "-o",
        file_set.preprocessed_source().to_str().unwrap()
    ]).output()?;

    let source = fs::read_to_string(file_set.preprocessed_source())?;
    fs::remove_file(file_set.preprocessed_source())?;
    let tokenizer = Tokenizer::new(source.as_str());

    if cli.lex {
        let tokens: Result<Vec<_>, _> = tokenizer.collect();
        return match tokens {
            Ok(_) => Ok(()),
            Err(err) => Err(CompilerError::LexerError(err))
        };
    }

    let ast = parse(&mut tokenizer.peekable()).map_err(|err| CompilerError::ParserError(err))?;

    if cli.parse {
        return Ok(());
    }

    let mut ir_emitter = TackEmitter::new();
    let ir = ir_emitter.emit_program(&ast);

    if cli.tacky {
        return Ok(());
    }

    let instructions = {
        let asm = codegen(&ir);

        let asm_passes: Vec<Box<dyn AsmPass>> = vec![
            Box::new(AsmPassPseudoRegister::new())
        ];

        asm_passes.into_iter().fold(asm, |instructions, mut pass| { pass.run(instructions) })
    };

    println!("{:#?}", instructions);

    if cli.codegen {
        return Ok(());
    }

    //
    // let mut writer = CodeWriter::new();
    // emit(&mut writer, &instructions);
    //
    //
    // println!("Output path: {:?}", file_set.assembly_file);
    // let mut output_file = File::create(file_set.assembly_file())?;
    // output_file.write_all(writer.as_str().as_bytes())?;
    //
    // std::process::Command::new("gcc").args([file_set.assembly_file().to_str().unwrap(), "-o", file_set.executable().to_str().unwrap()]).output()?;
    //
    // std::fs::remove_file(file_set.assembly_file)?;

    Ok(())
}
