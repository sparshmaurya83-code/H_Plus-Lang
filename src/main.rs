mod lexer;
mod parser;
mod ast;
mod interpreter;

use std::io::{self, Write};
use std::fs;

use lexer::tokenize;
use parser::Parser;
use interpreter::Interpreter;

fn run_code(code: &str, interpreter: &mut Interpreter) {
    let tokens = tokenize(code);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    interpreter.run(ast);
}

fn main() {
    let mut interpreter = Interpreter::new();

    println!("🔥 H+ REPL (type 'exit' to quit)");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        run_code(&input, &mut interpreter);
    }
}