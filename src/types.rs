use evm::Opcode;
use std::fmt;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Bytes32(pub [u8;4]);
type PushData = String;
pub type Instruction = (Opcode, Option<PushData>);

struct Instr(Instruction);

impl fmt::Display for Instr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", "TODO")
    }
}
