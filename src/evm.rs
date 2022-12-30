use crate::memory::Memory;
use crate::stack::Stack;
use crate::storage::Storage;
use crate::types::Instruction;

#[allow(dead_code)]
pub struct EVM {
    stack: Stack,
    memory: Memory,
    storage: Storage,
}

impl EVM {
    pub fn new(&self) -> Self {
        EVM {
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
        }
    }

    pub fn execute_bytecode(&self, instructions: &Vec<Instruction>) {
        for instruction in instructions.iter() {
            match instruction.0.as_u8() {
                0x60..=0x7F => todo!("PUSH, must account for pushdata"),
                0x80..=0x8F => todo!("DUP, must read from stack"),
                0x90..=0x9f => todo!("SWAP, must read from stack"),

                _ => todo!("im hungry rn"),
            }
        }
    }

    fn mstore(&self) {
        todo!()
    }

    fn mload(&self) {
        todo!()
    }

    fn sstore(&self) {
        todo!()
    }

    fn push(&self) {
        todo!()
    }

    fn pop() {
        todo!()
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
