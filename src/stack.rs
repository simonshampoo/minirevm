use crate::storage::Bytes32;

pub struct Stack {
    stack: Vec<Bytes32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            stack: Vec::<Bytes32>::new(),
        }
    }

    pub fn push(&mut self, value: Bytes32) {
        if self.stack.len() > 1024 {
            panic!("Stack too deep")
        }
        self.stack.push(value);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }
}
