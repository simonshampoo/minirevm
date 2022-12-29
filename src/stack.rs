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
        if self.size() > 1024 {
            panic!("Stack Overflow")
        }

        // I need to pad the bytes im pretty sure. this is INCOMPLETE
        self.stack.push(value);
    }

    pub fn pop(&mut self) {
        if self.size() < 1 {
            panic!("Stack underflow")
        }

        self.stack.pop();
    }

    fn size(&self) -> usize {
        self.stack.len()
    }

    fn peek(&self) -> Option<Bytes32> {
        if self.size() < 1 {
            return None;
        }
        Some(self.stack[self.size() - 1])
    }

    pub fn dup(&mut self, position: usize) {
        if self.size() > 1023 {
            todo!("good error message here")
        }
        if self.size() < position {
            todo!("good error message here")
        }
        if position > 16 {
            todo!("good error message here")
        }

        let peek_index = self.size() - position - 1;
        self.stack.push(self.stack[peek_index]);
    }

    pub fn swap(&mut self, position: usize) {
        let sz = self.size();
        self.stack.swap(position, sz);
    }
}
