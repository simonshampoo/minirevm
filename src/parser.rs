use crate::types::Instruction;
use evm::Opcode;
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

        if codesize == 0 {}

        for i in 0..=codesize - 2 {
            let opcode = Opcode(bytecode[i..i + 2].parse().unwrap());

            let mut pushdata_len: usize = 0;
            if let Some(len) = opcode.is_push() {
                pushdata_len = len as usize;
            }

            if pushdata_len != 0 {}
            self.instructions
                .push((Opcode { 0: opcode.0 }, Some(String::from("IDK"))));

            println!("OPCODE: {}", opcode.0);
            /*

            now we check if the opcode is one that takes a stack param.
            and in the case that it does, we see how many it takes.

            */
        }
    }
}
