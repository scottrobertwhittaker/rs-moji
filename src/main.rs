mod interpreter;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let code: &'static str =
        "👍👍👍👍👍👍👍👍👍👍🔃👉👍👍👍👍👍👍👍👉👍👍👍👍👍👍👍👍👍👍👉👍👍👍👉👍👈👈👈👈👎🔙👉👍👍💻👉👍💻👍👍👍👍👍👍👍💻💻👍👍👍💻👉👍👍💻👈👈👍\
         👍👍👍👍👍👍👍👍👍👍👍👍👍👍💻👉💻👍👍👍💻👎👎👎👎👎👎💻👎👎👎👎👎👎👎👎💻👉👍💻👉💻"; // = ARGF.read.chomp;

    let tokens = lexer::lex("test");
    for i in tokens.iter() {
        println!("* {}", i);
    }

    let ast = parser::parse(tokens);

    let runtime = runtime::new();

    let interpreter = interpreter::Interpreter::new(runtime);
    interpreter.evaluate(ast);
}

#[test]
fn breathing_test() {
    println!("Success!");
}
