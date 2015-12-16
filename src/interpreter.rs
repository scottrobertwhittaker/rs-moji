use runtime;

pub struct Interpreter;

impl Interpreter {
    pub fn new(runtime: runtime::Runtime) -> Interpreter {
        println!("Initializing interpreter with runtime {}",
                 runtime.x());
        Interpreter
    }

    pub fn evaluate(&self, ast: Vec<&str>) {
        println!("Evaluating {}...", ast.join(", "));
    }
}