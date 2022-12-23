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
        while i < codesize - 2 {
            // how tf to parse hexadecimal strings as hexadecimal numbers
            println!(
                "bytecode parse test: {:?}",
                i64::from_str_radix(&bytecode[i..i + 2], 16)
            );
            let opcode = Opcode {
                0: i64::from_str_radix(&bytecode[i..i + 2], 16).unwrap() as u8,
            };

            println!("OPCODE {}", &opcode.0);
            let mut pushdata_len: usize = 0;
            if let Some(len) = opcode.is_push() {
                pushdata_len = (len * 2) as usize;
                println!("do we ever go here? {}", pushdata_len);
                // NEWSFLASH BUDDY: WE DONT AND ITS BECAUSE HEX STRING AND u8 AND SHIT IS CONFUSING ME
            }
            if pushdata_len != 0 {
                let immediate_val = &bytecode[i + 2..pushdata_len];
                self.instructions
                    .push((Opcode { 0: opcode.0 }, Some(String::from(immediate_val))));
                i += pushdata_len;
            } else {
                i += 2;
                self.instructions.push((Opcode { 0: opcode.0 }, None));
            }
        }
        println!("Instructions received: {:?}", self.instructions);

        &self.instructions
    }
}
