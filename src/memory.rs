use crate::types::Bytes32;

pub struct Memory {
    pub memory: Vec<Bytes32>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: Vec::<Bytes32>::new(),
        }
    }

    pub fn mstore(&mut self, val: Bytes32) {
        self.memory.push(val);
    }
    
    // idk how this is gonna work with strings and other dynamic memory
    pub fn mload(&self, offset: usize) -> &[Bytes32] {
        &self.memory[offset..(offset + 32)%32] // also this is probably wrong
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;

        assert_eq!(result, 4);
    }
}
