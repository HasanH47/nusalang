// src/main.rs
mod lexer;
mod parser;
mod ast;
mod eval;

use std::fs;
use std::env;
use lexer::tokenize;
use parser::parse;
use eval::eval;

fn main() {
    println!("Welcome to NusaLang CLI Interpreter (Alpha)");

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: nusa <filename.nusa>");
        return;
    }

    let filename = &args[1];
    let source = fs::read_to_string(filename).expect("Failed to read source file");

    match tokenize(&source) {
        Ok(tokens) => {
            match parse(&tokens) {
                Ok(ast) => {
                    match eval(&ast) {
                        Ok(_) => {},
                        Err(e) => eprintln!("Runtime Error: {}", e),
                    }
                }
                Err(e) => eprintln!("Parse Error: {}", e),
            }
        }
        Err(e) => eprintln!("Lexing Error: {}", e),
    }
}
