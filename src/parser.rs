#![feature(is_some_with)]

use evm::Opcode;
use std::collections::BTreeMap;
type PushData = String;
use std::str;
pub struct Parser {
    instructions: BTreeMap<Opcode, Option<PushData>>,
}

impl Parser {
    fn new() -> Self {
        Parser {
            instructions: BTreeMap::new(),
        }
    }
    fn parse(bytecode: &String) {
        let free_mem_ptr_i12n = &bytecode.as_bytes()[..2]; // this shit better be 60 bro
        
        for (i, c) in bytecode.chars().enumerate() {
            let opcode = str::from_utf8(&bytecode.as_bytes()[i..i + 2]);

            if !opcode.is_ok() {
                panic!("FUCK")
            }

            let free_mem_ptr_initialization = match opcode {
                Ok(_) => "60",
                Err(error) => panic!(
                    "Didn't initialize free memory pointer in your bytecode. Go to jail. {}",
                    error
                ),
            };
            if opcode.unwrap() != "60" {
                todo!("idk haven't done this yet. usually solidity contracts start with 6040 to initialize free mem pointer so u doing some freaky stuff")
            }

            match opcode {}
        }
    }
}
