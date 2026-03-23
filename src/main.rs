use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: hplus <file>");
        return;
    }

    let code = fs::read_to_string(&args[1])
        .expect("Failed to read file");

    let tokens = tokenize(&code);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.run(ast);
}