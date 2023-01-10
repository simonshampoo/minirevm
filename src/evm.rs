use crate::memory::Memory;
use crate::opcodes;
use crate::stack::Stack;
use crate::storage::Storage;
use crate::types::{Bytes32, Instruction};
use crate::utils::match_stackop_n;
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
                //                0x01 => todo!("ADD"),
                //                0x02 => todo!("MUL"),
                //                0x03 => todo!("SUB"),
                //                0x04 => todo!("DIV"),
                //                0x05 => todo!("SDIV"),
                //                0x06 => todo!("MOD"),
                //                0x07 => todo!("SMOD"),
                //                0x08 => todo!("ADDMOD"),
                //                0x09 => todo!("MULMOD"),
                //                0x0A => todo!(""),
                //                0x0B => todo!(""),
                //                0x10 => todo!(""),
                //                0x11 => todo!(""),
                //                0x12 => todo!(""),
                //                0x13 => todo!(""),
                //                0x14 => todo!(""),
                //                0x15 => todo!(""),
                //                0x16 => todo!(""),
                //                0x17 => todo!(""),
                //                0x18 => todo!(""),
                //                0x19 => todo!(""),
                opcodes::PUSH1..=opcodes::PUSH32 => {
                    // PUSH operations
                    println!(
                        "PUSH{} 0x{}",
                        instruction.0.as_u8() - 95,
                        instruction.1.as_ref().unwrap()
                    );
                    self.stack.push(Bytes32(
                        hex::decode(instruction.1.as_ref().unwrap())
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    ));
                }

                opcodes::POP => {
                    self.stack.pop();
                }
                opcodes::DUP1..=opcodes::DUP16 => {
                    let dup_n = match_stackop_n(Opcode(instruction.0.as_u8()));
                    println!("dup_n = {}", dup_n);
                    self.stack.dup(dup_n);
                }

                opcodes::SWAP1..=opcodes::SWAP16 => {
                    let swap_n = match_stackop_n(Opcode(instruction.0.as_u8()));
                    println!("SWAP{}", instruction.0.as_u8() - 143,);
                    self.stack.swap(swap_n);
                }
                opcodes::MSTORE => {
                    let offset = self.stack.pop();
                    let data = self.stack.pop();
                    self.mstore(offset, data);
                }
                opcodes::SSTORE => {
                    let key = self.stack.pop();
                    let value = self.stack.pop();
                    self.sstore(key, value);
                }
                _ => {
                    println!("## STACK STATE ##");
                    self.print_stack()
                }
            }
        }
    }

    fn stop() {
        std::process::exit(0)
    }

    fn add(a: Bytes32, b: Bytes32) {
       

    }

    fn mstore(&mut self, offset: Bytes32, data: Bytes32) {
        self.memory.mstore(offset, data);
    }

    fn mload(&self, offset: Bytes32) {
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
