use crate::types::Bytes32;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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

        self.stack.push(value);
    }

    pub fn pop(&mut self) -> Bytes32 {
        if self.size() < 1 {
            panic!("Stack underflow")
        }

        self.stack.pop().unwrap()
    }

    pub fn size(&self) -> usize {
        self.stack.len()
    }

    pub fn dup(&mut self, position: usize) {
        if self.size() > 1023 {
            println!("stack too deep")
        }
        if self.size() < position {
            println!(
                "size is {} , position is {} ... out of bounds error",
                self.size(),
                position
            )
        }
        if position > 16 {
            println!("not supported")
        }

        let peek_index = self.size() - position - 1;
        if (self.size() < peek_index) {
            panic!("{} FUCKKK", peek_index)
        }
        let duplicated_val = self.stack[peek_index].clone();
        self.stack.push(duplicated_val);
    }

    pub fn swap(&mut self, position: usize) {
        let sz = self.size();
        if position > sz {
            panic!("not enough stack items")
        }
        self.stack.swap(position, sz - 1);
    }
}

impl fmt::Display for Stack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.stack
            .iter()
            .fold(Ok(()), |r, b| r.and_then(|_| writeln!(f, "{}", b)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut stack = Stack::new();

        let value: Bytes32 = Bytes32(vec![1, 2, 3, 4]);

        stack.push(value);

        assert_eq!(stack.stack[0], vec![1, 2, 3, 4])
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();

        let value: Bytes32 = Bytes32(vec![1, 2, 3, 4]);

        stack.push(value);
        stack.pop();

        assert_eq!(stack.size(), 0)
    }

    #[test]
    fn test_swap() {
        let mut stack = Stack::new();

        let value: Bytes32 = [1, 2, 3, 4];
        let value1: Bytes32 = [4, 3, 2, 1];
        stack.push(value);
        stack.push(value1);
        stack.swap(0);

        assert_eq!(stack.stack[0], value1)
    }
    #[test]
    fn test_dup() {
        let mut stack = Stack::new();

        let value: Bytes32 = [1, 2, 3, 4];
        stack.push(value);
        stack.dup(0);
        println!("{:?}", stack.stack);
        assert!(stack.size() == 2 && stack.stack[0] == stack.stack[1])
    }
}
