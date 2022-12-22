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

        for i in 0..=bytecode.len() - 2 {
            let opcode = str::from_utf8(&bytecode.as_bytes()[i..i + 2]).unwrap();
            i += 2;
            // convert opcode to type Opcode
            // be able to parse the hex value ??
            // idk types lol
            let jumps = match opcode {
                0x60..=0x7f => match_push_n(opcode),
                _ => usize::MAX,
            };

            if jumps == usize::MAX {
                panic!("what the fuck")
            }

            /*

            match opcode {
                <opcode_that_takes_stack_input> => look at stack and and continue otherwise
                <opcode_that_doesnt_take_stack_input> =>
            }


            */
            self.instructions.push((
                Opcode {
                    0: opcode.unwrap().as_ptr() as u8,
                },
                Some(vec![String::from("IDK")]),
            ));

            println!("OPCODE: {}", opcode.unwrap());
            /*

            now we check if the opcode is one that takes a stack param.
            and in the case that it does, we see how many it takes.

            */
        }
    }
}
