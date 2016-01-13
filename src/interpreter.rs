use parser;
use runtime;

use std::char;
use std::collections::HashMap;
use std::io;

pub struct Interpreter {
    runtime: runtime::Runtime,
    loop_map: HashMap<usize, usize>
}

// TBD Is this customarily in the `impl` or outside it?
pub fn new(runtime: runtime::Runtime) -> Interpreter {
    Interpreter { runtime: runtime,
                  loop_map: HashMap::new() }
}

impl Interpreter {
    /// Execute the specified `ast`.
    pub fn evaluate(&mut self, ast: Vec<parser::Node>) -> &mut Interpreter {
        use parser::Node;

        let mut loop_stack: Vec<usize> = vec![];

        for (index, node) in ast.iter().enumerate() {
            match *node {
                Node::Loop => loop_stack.push(index),
                Node::EndLoop => {
                    let begin = loop_stack.pop().expect("Unmatched loop end");
                    self.loop_map.insert(begin, index);
                }
                _ => {}
            }
        }

        if !loop_stack.is_empty() {
            panic!("Unmatched loop begin");
        }

        while self.runtime.instruction_pointer() != ast.len() {
            match ast[self.runtime.instruction_pointer()] {
                Node::PointLeft =>  self.runtime.decrement_data_pointer(),
                Node::PointRight => self.runtime.increment_data_pointer(),
                Node::ThumbsUp =>   self.runtime.increment_data(),
                Node::ThumbsDown => self.runtime.decrement_data(),
                Node::Save => {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input);
                    self.runtime.set_data(
                                   input.trim().bytes().next().unwrap() as i8);

                }
                Node::Display => {
                    let character =
                           char::from_u32(self.runtime.data() as u32).unwrap();
                    print!("{}", character)
                }
                Node::Loop => {
                    if self.runtime.data() == 0 {
                        let begin = &self.runtime.instruction_pointer();
                        let end = self.loop_map.get(begin).unwrap();
                        self.runtime.set_instruction_pointer(*end);
                    }
                }
                Node::EndLoop => {
                    if self.runtime.data() != 0 {
                        let end = &self.runtime.instruction_pointer();
                        let mut begin = None;
                        for (b, e) in self.loop_map.iter() {
                            if end == e {
                                begin = Some(b);
                                break;
                            }
                        }
                        self.runtime.set_instruction_pointer(*begin.unwrap());
                    }
                }
                Node::None => {}
            }
            self.runtime.increment_instruction_pointer();
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use parser::Node;
    use runtime;

    use std::collections::HashMap;

    // TBD Is it possible to specify a preamble that's run before each test?

    #[test]
    fn test_eval_point_right_node() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let ast = vec![Node::PointRight];
        interpreter.evaluate(ast);
        assert_eq!(1, interpreter.runtime.data_pointer())
    }

    #[test]
    fn test_eval_point_left_node() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let ast = vec![Node::PointRight, Node::PointRight, Node::PointLeft];
        interpreter.evaluate(ast);
        assert_eq!(1, interpreter.runtime.data_pointer())
    }

    #[test]
    fn test_eval_thumbs_up_node() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let ast = vec![Node::ThumbsUp];
        interpreter.evaluate(ast);
        assert_eq!(1, interpreter.runtime.data())
    }

    #[test]
    fn test_eval_thumbs_down_node() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let ast = vec![Node::ThumbsDown];
        interpreter.evaluate(ast);
        assert_eq!(-1, interpreter.runtime.data())
    }

    #[test]
    fn test_eval_display_node() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let ast = vec![Node::ThumbsUp; 72];
        interpreter.evaluate(ast);
        // TBD Assert that "H" was printed to `stdout`.
    }

    #[test]
    fn test_eval_save_node() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        // TBD Feed 'a' to `stdin` and verify that the runtime data is `97`.
     // let ast = vec![Node::Save];
     // interpreter.evaluate(ast);
     // assert_eq!(97, interpreter.runtime.data())
    }

    #[test]
    fn test_eval_loop_node() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let ast = vec![Node::Loop, Node::ThumbsUp, Node::EndLoop];
        interpreter.evaluate(ast);
        assert_eq!(0, interpreter.runtime.data())
    }

    #[test]
    fn test_eval_end_loop_node() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        // Loop twice
        let ast = vec![Node::ThumbsUp,
                       Node::ThumbsUp,

                       Node::Loop,

                       // Move right once, increment, move left
                       Node::PointRight,
                       Node::ThumbsUp,
                       Node::PointLeft,

                       // Decrement so we leave the loop
                       Node::ThumbsDown,

                       Node::EndLoop,

                       // Move right so we can see the 1 we did in the loop
                       Node::PointRight];
        interpreter.evaluate(ast);
        assert_eq!(2, interpreter.runtime.data())
    }

    #[test]
    fn test_loop_map() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let mut map = HashMap::new();
        map.insert(0, 1);

        let ast = vec![Node::Loop, Node::EndLoop];
        interpreter.evaluate(ast);
        assert_eq!(map, interpreter.loop_map)
    }

    #[test]
    fn test_nested_loop_map() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let mut map = HashMap::new();
        map.insert(0, 3);
        map.insert(1, 2);

        let ast = vec![Node::Loop, Node::Loop, Node::EndLoop, Node::EndLoop];
        interpreter.evaluate(ast);
        assert_eq!(map, interpreter.loop_map)
    }

    #[test]
    #[should_panic(expected = "Unmatched loop begin")]
    fn test_uneven_loop() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let ast = vec![Node::Loop];
        interpreter.evaluate(ast);
    }

    #[test]
    #[should_panic(expected = "Unmatched loop end")]
    fn test_uneven_end_loop() {
        let runtime = runtime::new();
        let mut interpreter = new(runtime);

        let ast = vec![Node::EndLoop];
        interpreter.evaluate(ast);
    }
}
