mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::fs;
use lexer::tokenize;
use parser::Parser;
use interpreter::Interpreter;

fn main() {
    let code = fs::read_to_string("examples/hello.hplus")
        .expect("Failed to read file");

    let tokens = tokenize(&code);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.run(ast);
}