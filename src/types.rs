use evm::Opcode;

type PushData = String;
pub type Instruction = (Opcode, Option<PushData>);
