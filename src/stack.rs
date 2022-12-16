struct Stack {
    stack: Vec<Bytes32>,
}

impl Stack {
    fn new(&self) -> Self {
        Stack { stack: Vec<u8>::new() }
    }

    fn push(&mut self, value: Bytes32) -> Self {
        self.stack.push(value)
    }
}
