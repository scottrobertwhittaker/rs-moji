mod interpreter;
mod lexer;
mod parser;
mod runtime;

fn main() {
    let code: &'static str =
        "👍👍👍👍👍👍👍👍👍👍🔃👉👍👍👍👍👍👍👍👉👍👍👍👍👍👍👍👍👍👍👉👍👍👍👉👍👈👈👈👈👎🔙👉👍👍💻👉👍💻👍👍👍👍👍👍👍💻💻👍👍👍💻👉👍👍💻👈👈👍\
         👍👍👍👍👍👍👍👍👍👍👍👍👍👍💻👉💻👍👍👍💻👎👎👎👎👎👎💻👎👎👎👎👎👎👎👎💻👉👍💻👉💻"; // = ARGF.read.chomp;

    let tokens: Vec<char> = lexer::lex(code);

    let ast = parser::parse(tokens);

    let runtime = runtime::new();

    let interpreter = interpreter::Interpreter::new(runtime);
    interpreter.evaluate(ast);
}
