pub struct Runtime {
    x: i32,
}

pub fn new() -> Runtime {
    Runtime { x: 123 }
}

impl Runtime {
    pub fn x(&self) -> i32 {
        self.x
    }
}
