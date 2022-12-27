use crate::storage::Bytes32;
use std::io::{Error, ErrorKind};

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
            panic!("FUCKKKKKK")
        }
        self.stack.push(value);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn dup(&mut self, position: usize) -> Result<Bytes32, &'static str> {
        if self.stack.len() < position {
            return Err("fuck");
        }
        if position > 16 {
            return Err("fuck");
        }

        let peek_index = -1 * position as i8;

        self.stack.push(self.stack[peek_index as usize]);

        self.stack.swap(0, position);

        Ok(self.stack[peek_index as usize])
    }
}
