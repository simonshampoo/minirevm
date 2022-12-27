use crate::storage::Bytes32;
use std::io::{Error, ErrorKind};

pub struct Stack {
    stack: Vec<Bytes32>,
}

impl Stack {
    pub fn new() -> Self {
        Stack { stack: Vec::<Bytes32>::new(),
        }
    }

    pub fn push(&mut self, value: Bytes32) {
        if self.stack.len() > 1024 {
            panic!("Stack Overflow")
        }

        // I need to pad the bytes im pretty sure. this is INCOMPLETE
        self.stack.push(value);
    }

    pub fn pop(&mut self) {
        
        if (self.stack.len() < 1) {
            panic!("Stack underflow")
        }

        self.stack.pop();
    }

    fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn dup(&mut self, position: usize) -> Result<Bytes32, &'static str> {

        if self.size() > 1023 {
            return Err("stack overflow")
        }
        if self.size() < position {
            return Err("fuck");
        }
        if position > 16 {
            return Err("fuck");
        }

        let peek_index = self.size()-position-1;
        self.stack.push(self.stack[peek_index]);

        self.stack.swap(0, position);

        Ok(self.stack[peek_index as usize])
    }
}
