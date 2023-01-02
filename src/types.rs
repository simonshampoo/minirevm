use evm::Opcode;
use std::fmt;

pub type Bytes32 = [u8; 4];
type PushData = String;
pub type Instruction = (Opcode, Option<PushData>);

struct Instr(Instruction);

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "TODO")
    }
}
