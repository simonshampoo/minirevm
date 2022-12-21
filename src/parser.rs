use evm::Opcode;
use std::collections::BTreeMap;
type PushData = String;
use std::str;
pub struct Parser {
    instructions: BTreeMap<Opcode, Option<PushData>>,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            instructions: BTreeMap::new(),
        }
    }
    pub fn parse(&self, bytecode: &String) {
        let codesize = bytecode.len();

        if codesize == 0 {
            panic!("no bytecode provided")
        }
        let mut i = 0;

        while i < bytecode.len() - 2 {
            let opcode = str::from_utf8(&bytecode.as_bytes()[i..i + 2]);
            i += 2;

            println!("OPCODE: {}", opcode.unwrap());
            /*

            now we check if the opcode is one that takes a stack param.
            and in the case that it does, we see how many it takes.

            */
        }
    }
}
