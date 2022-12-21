use evm::Opcode;
use std::str;

type PushData = String;
type Instruction = (Opcode, Option<Vec<PushData>>);
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
        let mut i = 0;

        while i < bytecode.len() - 2 {
            let opcode = str::from_utf8(&bytecode.as_bytes()[i..i + 2]);
            i += 2;
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
