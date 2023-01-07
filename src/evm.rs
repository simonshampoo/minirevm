use crate::memory::Memory;
use crate::stack::Stack;
use crate::storage::Storage;
use crate::types::{Bytes32, Instruction};
use crate::utils::{decode_hex, encode_hex, match_stackop_n};
use evm::Opcode;

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
                        "PUSH{} OPERATION AND WE'RE PUSHING {:?}",
                        //i32::from_str_radix(instruction.1.as_ref().unwrap(), 16)
                        //   .unwrap()
                        //  .to_be_bytes()
                        instruction.0.as_u8() - 95,
                        &instruction.1
                    );
                    self.stack.push(Bytes32 {
                        0: decode_hex(&instruction.1.as_ref().unwrap())
                            .unwrap()
                            .as_slice()
                            .try_into().unwrap(),
                    });
                }
                0x80..=0x8F => {
                    let dup_n = match_stackop_n(Opcode {
                        0: instruction.0.as_u8(),
                    });

                    self.stack.dup(dup_n);
                }

                0x90..=0x9f => {
                    let swap_n = match_stackop_n(Opcode {
                        0: instruction.0.as_u8(),
                    });
                    self.stack.swap(swap_n);
                }
                _ => (),
            }
        }
    }

    fn mstore(&mut self, data: Bytes32) {
        self.memory.mstore(data);
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
        println!("{:?}", self.stack)
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
