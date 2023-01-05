use crate::memory::Memory;
use crate::stack::Stack;
use crate::storage::Storage;
use crate::types::{Bytes32, Instruction};

#[allow(dead_code)]
pub struct EVM {
    stack: Stack,
    memory: Memory,
    storage: Storage,
}

#[allow(dead_code)]
impl EVM {
    pub fn new(&self) -> Self {
        EVM {
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
        }
    }

    pub fn execute_bytecode(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions.iter() {
            match instruction.0.as_u8() {
                0x60..=0x7F => {
                    self.stack.push(
                        instruction
                            .1
                            .as_ref()
                            .unwrap()
                            .as_bytes()
                            .try_into().unwrap()
                    );
                }
                0x80..=0x8F => todo!("DUP, must read from stack"),
                0x90..=0x9f => todo!("SWAP, must read from stack"),

                _ => todo!("im hungry rn"),
            }
        }
    }

    fn mstore(&mut self, data: Bytes32) {
        self.memory.mstore(data);
    }

    fn mload(&self, offset: usize) -> &Bytes32 {
        self.memory.mload(offset)
    }

    fn sstore(&mut self, key: Bytes32, value: Bytes32) {
        self.storage.sstore(key, value);
    }

    fn push(&mut self, data: Bytes32) {
        self.stack.push(data);
    }

    fn pop(&mut self) {
        self.stack.pop();
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
