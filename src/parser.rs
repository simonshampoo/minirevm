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
        println!(
            "=================================================================================\nBytecode received: {}",
            &bytecode
        );
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

            i += 2;
            let mut pushdata_len: usize = 0;
            if let Some(len) = opcode.is_push() {
                pushdata_len = (len * 2) as usize;
            }
            if pushdata_len != 0 {
                let immediate_val = &bytecode[i..i + pushdata_len];
                self.instructions
                    .push((Opcode { 0: opcode.0 }, Some(String::from(immediate_val))));
                i += pushdata_len;
            } else {
                self.instructions.push((Opcode { 0: opcode.0 }, None));
            }
        }
        //  println!("Instructions received: {:x?}", self.instructions);
        let max_for_padding = self
            .instructions
            .iter()
            .map(|(_, v)| v)
            .max()
            .unwrap()
            .as_ref()
            .unwrap()
            .len();
        println!(
            "================================================================================="
        );
        for instruction in self.instructions.iter() {
            match &instruction.1 {
                Some(pushdata) => println!(
                    "0x{:x?} {} {: >max$}({} bytes)",
                    instruction.0.as_u8(),
                    pushdata,
                    "",
                    pushdata.len() / 2,
                    max = 33 + max_for_padding - pushdata.len()
                ),
                None => println!("0x{:x?}", instruction.0.as_u8()),
            }
        }

        println!(
            "================================================================================="
        );
        &self.instructions
    }
}
