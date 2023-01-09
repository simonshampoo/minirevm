use evm::Opcode;
use std::{fmt, ops};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
pub struct Bytes32(pub Vec<u8>);
type PushData = String;
pub type Instruction = (Opcode, Option<PushData>);

struct Instr(Instruction);

impl fmt::Display for Bytes32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .fold(Ok(()), |r, b| r.and_then(|_| write!(f, "{:x?}", b)))
    }
}

impl Bytes32 {
    pub fn new() -> Self {
        Bytes32(Vec::<u8>::new())
    }
}

impl ops::Add for Bytes32 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Bytes32(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(b0, b1)| b0 - b1)
                .collect(),
        )
    }
}

impl ops::Sub for Bytes32 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Bytes32(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(b0, b1)| b0 - b1)
                .collect(),
        )
    }
}
impl ops::Mul for Bytes32 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Bytes32(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(b0, b1)| b0 - b1)
                .collect(),
        )
    }
}

impl ops::Div for Bytes32 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Bytes32(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(b0, b1)| b0 & b1)
                .collect(),
        )
    }
}

impl ops::BitAnd for Bytes32 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Bytes32(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(b0, b1)| b0 & b1)
                .collect(),
        )
    }
}
