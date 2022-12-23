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
        let mut i = 0;
        while i < codesize - 2 {
            let opcode = Opcode(bytecode[i..i + 2].parse().unwrap());

            let mut pushdata_len: usize = 0;
            if let Some(len) = opcode.is_push() {
                pushdata_len = len as usize;
            }

            if pushdata_len != 0 {
                let immediate_val = &bytecode[i + 2..pushdata_len];
                self.instructions
                    .push((Opcode { 0: opcode.0 }, Some(String::from(immediate_val))));
                i += pushdata_len;
            } else {
                i += 1;
            }
            println!("OPCODE: {}", opcode.0);
        }

        println!("Instructions received: {:?}", self.instructions);
    }
}
