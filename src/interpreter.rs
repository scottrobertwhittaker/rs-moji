use parser;
use runtime;

pub struct Interpreter;

impl Interpreter {
    pub fn new(runtime: runtime::Runtime) -> Interpreter {
        println!("Initializing interpreter with runtime {:?}", runtime);
        Interpreter
    }

    pub fn evaluate(&self, ast: Vec<parser::Node>) {
        println!("Evaluating...");
    }
}
