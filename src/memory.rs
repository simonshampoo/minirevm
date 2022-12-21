use crate::storage::Bytes32;

pub struct Memory {
    pub memory: Vec<Bytes32>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: Vec::<Bytes32>::new(),
        }
    }

    pub fn push(&mut self, val: Bytes32) {
        self.memory.push(val);
    }
}
