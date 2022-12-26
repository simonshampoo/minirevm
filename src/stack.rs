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
            panic!("FUCKKKKKK")
        }
        self.stack.push(value);
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn dup(&mut self, x: usize) {
        if self.stack.len() < x {
            panic!("stack underflow")
        }
        if x > 16 {
            panic!("not possible")
        }

        self.stack.swap(0, x);
        if x == 1 {
            self.stack.push(self.stack[0]);

            self.stack.push(self.stack[x]);
        } else {
            todo!("need to implement algorithm for the other DUP opcodes cuz the stack output for them is weird")
        }
    }
}
