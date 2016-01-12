use std::fmt;

/// An item in the `Runtime` data array.
pub type Cell = i8;

/// A representation of the state of a Mojikun program, consisting of an array
/// of cells, an instruction pointer (i.e. program counter), and a data pointer
/// (i.e. index into the array of cells).
pub struct Runtime {
    instruction_pointer: usize,
    data_pointer: usize,
    data: Vec<Cell>
}

/// Creates an empty `Runtime` with 30000 cells initialized to zero.
pub fn new() -> Runtime {
    Runtime { instruction_pointer: 0,
              data_pointer: 0,
              data: vec![0; 30000] }
}

impl Runtime {
    // MANIPULATORS
    pub fn increment_instruction_pointer(&mut self) {
        self.instruction_pointer += 1;
    }

    pub fn set_instruction_pointer(&mut self, val: usize) {
        self.instruction_pointer = val;
    }

    pub fn increment_data_pointer(&mut self) {
        self.data_pointer += 1;
    }

    pub fn decrement_data_pointer(&mut self) {
        self.data_pointer -= 1;
    }

    pub fn set_data(&mut self, val: Cell) {
        self.data[self.data_pointer] = val
    }

    pub fn increment_data(&mut self) {
        self.data[self.data_pointer] += 1;
    }

    pub fn decrement_data(&mut self) {
        self.data[self.data_pointer] -= 1;
    }

    // ACCESSORS
    pub fn instruction_pointer(&self) -> usize {
        self.instruction_pointer
    }

    pub fn data_pointer(&self) -> usize {
        self.data_pointer
    }

    pub fn data(&self) -> Cell {
        self.data[self.data_pointer]
    }

}

impl fmt::Debug for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "{{ instruction_pointer: {}, data_pointer: {} }}",
               self.instruction_pointer,
               self.data_pointer)
    }
}

#[cfg(test)]
mod tests {
    use super::new;

    #[test]
    fn test_instruction_pointer() {
        assert_eq!(0, new().instruction_pointer)
    }

    #[test]
    fn test_data_pointer() {
        assert_eq!(0, new().data_pointer)
    }

    #[test]
    fn test_data() {
        assert_eq!(vec![0; 30000], new().data)
    }

    #[test]
    fn test_increment_data_pointer() {
        let mut runtime = new();
        runtime.increment_data_pointer();
        assert_eq!(1, runtime.data_pointer)
    }

    #[test]
    fn test_decrement_data_pointer() {
        let mut runtime = new();
        runtime.increment_data_pointer();
        runtime.increment_data_pointer();
        runtime.decrement_data_pointer();
        assert_eq!(1, runtime.data_pointer)
    }

    #[test]
    fn test_current_data() {
        assert_eq!(0, new().data())
    }

    #[test]
    fn test_increment_data() {
        let mut runtime = new();
        runtime.increment_data();
        assert_eq!(1, runtime.data())
    }

    #[test]
    fn test_decrement_data() {
        let mut runtime = new();
        runtime.decrement_data();
        assert_eq!(-1, runtime.data())
    }

    #[test]
    fn test_set_data() {
        let mut runtime = new();
        runtime.set_data(127);
        assert_eq!(127, runtime.data())
    }

    #[test]
    fn test_set_instruction_pointer() {
        let mut runtime = new();
        runtime.set_instruction_pointer(2);
        assert_eq!(2, runtime.instruction_pointer)
    }

    #[test]
    fn test_increment_instruction_pointer() {
        let mut runtime = new();
        runtime.increment_instruction_pointer();
        assert_eq!(1, runtime.instruction_pointer)
    }
}
