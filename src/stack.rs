struct Stack {
    stack: Vec<u8>,
}

impl Stack {
    fn new(&self) -> Self {
        Stack { stack: vec![] }
    }
}
