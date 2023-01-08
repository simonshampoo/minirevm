use crate::memory::Memory;
use crate::stack::Stack;
use crate::storage::Storage;
use crate::types::{Bytes32, Instruction};
use crate::utils::{decode_hex, match_stackop_n};
use evm::Opcode;
use hex::decode;
use std::u64;
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EVM {
    stack: Stack,
    memory: Memory,
    storage: Storage,
}

#[allow(dead_code)]
impl EVM {
    pub fn new() -> Self {
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
                    println!(
                        "PUSH{} 0x{}",
                        instruction.0.as_u8() - 95,
                        instruction.1.as_ref().unwrap()
                    );
                    self.stack.push(Bytes32 {
                        0: hex::decode(instruction.1.as_ref().unwrap())
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    });
                }
                0x80..=0x8F => {
                    let dup_n = match_stackop_n(Opcode {
                        0: instruction.0.as_u8(),
                    });
                    println!("DUP{}", instruction.0.as_u8() - 127,);
                    self.stack.dup(dup_n);
                }

                0x90..=0x9f => {
                    let swap_n = match_stackop_n(Opcode {
                        0: instruction.0.as_u8(),
                    });
                    println!("SWAP{}", instruction.0.as_u8() - 143,);
                    self.stack.swap(swap_n);
                }
                //                0x51 =>  {
                //                    self.mstore(
                //                        self.stack.pop(),
                //                        self.stack.pop(),
                //                    );
                //                },
                //                0x55 => {
                //                    self.sstore(self.stack.pop(), self.stack.pop());
                //                }
                _ => (),
            }
        }
    }

    fn mstore(&mut self, offset: Bytes32, data: Bytes32) {
        self.memory.mstore(offset, data);
    }

    fn mload(&self, offset: usize) {
        self.memory.mload(offset);
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

    pub fn print_stack(&self) {
        println!("{}", self.stack)
    }

    pub fn print_memory(&self) {
        println!("{:?}", self.memory)
    }

    pub fn print_storage(&self) {
        println!("{:?}", self.storage)
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
