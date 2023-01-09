use evm::Opcode;
use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Bytes32(pub Vec<u8>);
type PushData = String;
pub type Instruction = (Opcode, Option<PushData>);

struct Instr(Instruction);

impl fmt::Display for Bytes32 {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       self.0.iter().fold(Ok(()), |r, b| {
           r.and_then(|_| write!(f, "{:x?}", b))
       })
   } 
}

        
