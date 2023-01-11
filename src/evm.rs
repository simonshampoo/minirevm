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

    fn ADD(a: Bytes32, b: Bytes32) -> Bytes32 {
        a + b
    }

    fn SUB(a: Bytes32, b: Bytes32) -> Bytes32 {
        a - b
    }

    fn MUL(a: Bytes32, b: Bytes32) -> Bytes32 {
        a * b
    }
    fn DIV(a: Bytes32, b: Bytes32) -> Bytes32 {
        a / b
    }
    fn SDIV(a: Bytes32, b: Bytes32) -> Bytes32 {
        // TODO fix this? wtf is SDIV
        a / b
    }
    fn MOD(a: Bytes32, b: Bytes32) -> Bytes32 {
        a % b
    }
    fn SMOD(a: Bytes32, b: Bytes32) -> Bytes32 {
        a % b
    }
    fn ADDMOD(a: Bytes32, b: Bytes32, N: Bytes32) -> Bytes32 {
        (a + b) % N
    }
    fn MULMOD(a: Bytes32, b: Bytes32, N: Bytes32) -> Bytes32 {
        (a * b) % N 
    }
    fn EXP(a: Bytes32, exponent: Bytes32) -> Bytes32 {
        todo!()
    }
    fn SIGNEXTEND(b: Bytes32, x: Bytes32) -> Bytes32 {
        todo!()
    }
    fn LT(a: Bytes32, b: Bytes32) -> Bytes32 {
        a < b
    }
    fn GT(a: Bytes32, b: Bytes32) -> Bytes32 {
        a > b
    }
    fn SLT(a: Bytes32, b: Bytes32) -> Bytes32 {
        //TODO
        a < b
    }
    fn SGT(a: Bytes32, b: Bytes32) -> Bytes32 {
        //TODO
        a > b
    }
    fn EQ(a: Bytes32, b: Bytes32) -> Bytes32 {
        a == b
    }
    fn ISZERO(a: Bytes32) -> Bytes32 {
        //TODO
        if a == 0 {
            1
        } else {
            0
        }
    }
    fn AND(a: Bytes32, b: Bytes32) -> Bytes32 {
        a & b

    }
    fn OR(a: Bytes32, b: Bytes32) -> Bytes32 {
         a | b 

    }
    fn XOR(a: Bytes32, b: Bytes32) -> Bytes32 {
        a ^ b
    }
    fn NOT(a: Bytes32) -> Bytes32 {
        ~a 
    }
    fn BYTE(i: Bytes32, x: Bytes32) -> Bytes32 {}
    fn SHL(shift: Bytes32, value: Bytes32) -> Bytes32 {}
    fn SHR(shift: Bytes32, value: Bytes32) {}
    fn SAR(shift: Bytes32, value: Bytes32) {}
    fn SHA3(shift: Bytes32, value: Bytes32) {}
    fn ADDRESS() -> Bytes32 {
        println!("not yet implemented")
    }
    fn BALANCE(adress: Bytes32) -> Bytes32 {
        println!("not yet implemented") 
    }
    fn ORIGIN() -> Bytes32 {}
    fn CALLER() -> Bytes32 {}
    fn CALLVALUE() -> Bytes32 {}
    fn CALLDATALOAD(i: Bytes32) -> Bytes32 {}
    fn CALLDATASIZE() -> Bytes32 {}
    fn CALLDATACOPY(destOffset: Bytes32, offset: Bytes32, size: Bytes32) -> Bytes32 {}
    fn CODESIZE() -> Bytes32 {}
    fn COPDECOPY(destOffset: Bytes32, offset: Bytes32, size: Bytes32) -> Bytes32 {}
    fn GASPRICE() -> Bytes32 {}
    fn EXTCODESIZE(address: Bytes32) -> Bytes32 {}
    fn EXTCODECOPY(
        address: Bytes32,
        destOffset: Bytes32,
        offset: Bytes32,
        size: Bytes32,
    ) -> Bytes32 {
    }
    fn RETURNDATASIZE() -> Bytes32 {}
    fn RETURNDATACOPY(destOffset: Bytes32, offset: Bytes32, size: Bytes32) -> Bytes32 {}
    fn EXTCODEHASH(address: Bytes32) -> Bytes32 {}
    fn BLOCKHASH(blockNumber: Bytes32) -> Bytes32 {}
    fn COINBASE() -> Bytes32 {}
    fn TIMESTAMP() -> Bytes32 {}
    fn NUMBER() -> Bytes32 {}
    fn DIFFICULTY() -> Bytes32 {}
    fn GASLIMIT() -> Bytes32 {}
    fn CHAINID() -> Bytes32 {}
    fn SELFBALANCE() -> Bytes32 {}
    fn BASEFEE() -> Bytes32 {}
    fn MLOAD(a: Bytes32, b: Bytes32) -> Bytes32 {}
    fn MSTORE(&mut self, offset: Bytes32, value: Bytes32) {}
    fn MSTORE8(&mut self, offset: Bytes32, value: Bytes32) {
        // TODO
        // redo memory as to copy bytes rather than have each bytes32 occupy an index
        // endianness for mstore8
        // endianess for all other types
        todo!("COMPLETELY REDO MEMORY")
    }
    fn SLOAD(a: Bytes32, b: Bytes32) -> Bytes32 {}
    fn SSTORE(&mut self, a: Bytes32, b: Bytes32) -> Bytes32 {}
    fn JUMP(a: Bytes32, b: Bytes32) -> Bytes32 {}
    fn JUMPI(a: Bytes32, b: Bytes32) -> Bytes32 {}
    fn PC() -> Bytes32 {}
    fn MSIZE() -> Bytes32 {}
    fn GAS() -> Bytes32 {}
    fn JUMPDEST() -> Bytes32 {}
    fn PUSH1(&mut self, a: Bytes32) -> Bytes32 {}
    fn DUP1(&mut self, a: Bytes32) -> Bytes32 {}
    fn SWAP1(&mut self, a: Bytes32) -> Bytes32 {}
    fn LOG0(offset: Bytes32, size: Bytes32, topic1: Bytes32) -> Bytes32 {}
    fn CREATE(value: Bytes32, offset: Bytes32, size: Bytes32) -> Bytes32 {}
    fn CALL(
        gas: Bytes32,
        address: Bytes32,
        argsOffset: Bytes32,
        argsSize: Bytes32,
        retOffset: Bytes32,
        retSize: Bytes32,
    ) -> Bytes32 {
    }
    fn CALLCODE(
        gas: Bytes32,
        address: Bytes32,
        argsOffset: Bytes32,
        argsSize: Bytes32,
        retOffset: Bytes32,
        retSize: Bytes32,
    ) -> Bytes32 {
    }
    fn RETURN(&self, offset: Bytes32, size: Bytes32) -> Bytes32 {
       self.memory[offset..offset+size] 
    }
    fn DELEGATECALL(
        gas: Bytes32,
        address: Bytes32,
        argsOffset: Bytes32,
        argsSize: Bytes32,
        retOffset: Bytes32,
        retSize: Bytes32,
    ) -> Bytes32 {
    }
    fn CREATE2(value: Bytes32, offset: Bytes32, size: Bytes32, salt: Bytes32) -> Bytes32 {}
    fn STATICCALL(
        gas: Bytes32,
        address: Bytes32,
        argsOffset: Bytes32,
        argsSize: Bytes32,
        retOffset: Bytes32,
        retSize: Bytes32,
    ) -> Bytes32 {
    }
    fn REVERT(offset: Bytes32, size: Bytes32) -> Bytes32 {}
    fn INVALID() -> () {}
    fn SELFDESTRUCT(address: Bytes32) -> Bytes32 {}

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
