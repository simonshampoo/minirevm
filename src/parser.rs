use crate::types::Instruction;
use evm::Opcode;
use std::i64;
pub struct Parser {
    instructions: Vec<Instruction>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            instructions: Vec::new(),
        }
    }
    pub fn parse(&mut self, bytecode: &String) -> &Vec<Instruction> {
        let codesize = bytecode.len();

        if codesize == 0 {
            panic!("no bytecode provided");
        }
        let mut i = 0;
        while i < codesize {
            // how tf to parse hexadecimal strings as hexadecimal numbers
            let opcode = Opcode {
                0: i64::from_str_radix(&bytecode[i..i + 2], 16).unwrap() as u8,
            };

            println!("OPCODE 0x{:x?}", &opcode.0);

            i += 2;
            let mut pushdata_len: usize = 0;
            if let Some(len) = opcode.is_push() {
                pushdata_len = (len * 2) as usize;
                println!("pushdata_length: {}", pushdata_len);
            }
            if pushdata_len != 0 {
                let immediate_val = &bytecode[i..i + pushdata_len];
                println!("pushdata len != 0 and immediate val is {}", immediate_val);
                self.instructions
                    .push((Opcode { 0: opcode.0 }, Some(String::from(immediate_val))));
                i += pushdata_len;
            } else {
                self.instructions.push((Opcode { 0: opcode.0 }, None));
            }
        }
        println!("Instructions received: {:x?}", self.instructions);
        println!("============================");
        for instruction in self.instructions.iter() {
            match &instruction.1 {
                Some(pushdata) => println!("{:x?} {}", instruction.0, pushdata),
                None => println!("{:x?}", instruction.0),
            }
        }

        &self.instructions
    }
}
