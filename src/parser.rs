use crate::{types::Instruction, utils::match_push_n};
use evm::Opcode;
use hex::encode;
use std::str;
pub struct Parser {
    instructions: Vec<Instruction>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            instructions: Vec::new(),
        }
    }
    pub fn parse(&mut self, bytecode: &String) {
        let codesize = bytecode.len();

        if codesize == 0 {
            panic!("no bytecode provided")
        }

        for i in 0..=codesize - 2 {
            let opcode = Opcode(bytecode[i..i + 2].parse().unwrap());

            let jumps = 0;
            if let None = opcode.is_push() { // if we don't have a PUSH opcode, meaning we don't take an immediate value but possibly from the stack
            }

            //
            //            let jumps = match opcode {
            //                Opcode(0x60).as_u..=Opcode(0x7f)=> match_push_n(opcode),
            //                _ => usize::MAX,
            //            };

            if jumps == usize::MAX {
                panic!("what the fuck")
            }

            self.instructions
                .push((Opcode { 0: opcode.0 }, Some(vec![String::from("IDK")])));

            println!("OPCODE: {}", opcode.0);
            /*

            now we check if the opcode is one that takes a stack param.
            and in the case that it does, we see how many it takes.

            */
        }
    }
}
