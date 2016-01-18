#![allow(dead_code)]

use std::env;
use std::fs::File;
use std::io::Read;

mod interpreter;
mod lexer;
mod parser;
mod runtime;

fn main() {
    // TBD When are the right times to use `try!` vs. `unwrap`?

    let filename = env::args().nth(1).unwrap();
    let mut file = File::open(filename).unwrap();
    let mut code = String::new();
    file.read_to_string(&mut code).unwrap();

    let tokens: Vec<char> = lexer::lex(&code);
    let ast: Vec<parser::Node> = parser::parse(tokens);

    let runtime = runtime::new();
    let mut interpreter = interpreter::new(runtime);
    interpreter.evaluate(ast);
}
