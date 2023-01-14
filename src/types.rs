use evm::Opcode;
use num_bigint::BigUint;
use std::{fmt, ops};
use primitive_types::U256;

type PushData = String;
pub type Instruction = (Opcode, Option<PushData>);

#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd)]
pub struct Bytes32(pub Vec<u8>);

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

    pub fn from_bytes_be(b: Vec<u8>) -> Self {
        Bytes32(b.to_vec())
    }

    pub fn to_biguint(b: Self) -> num_bigint::BigUint {
        BigUint::from_bytes_be(b.0.as_slice())
    }

    pub fn from_biguint(b: BigUint) -> Self {
        Bytes32(b.to_bytes_be())
    }
}

impl ops::Add for Bytes32 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let a = BigUint::from_bytes_be(self.0.as_slice());

        let b = BigUint::from_bytes_be(other.0.as_slice());

        let sum = a + b;
        Bytes32(BigUint::to_bytes_be(&sum))
    }
}

impl ops::Sub for Bytes32 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let a = BigUint::from_bytes_be(self.0.as_slice());

        let b = BigUint::from_bytes_be(other.0.as_slice());

        let sum = a - b;
        Bytes32(BigUint::to_bytes_be(&sum))

        //        Bytes32(
        //            self.0
        //                .iter()
        //                .zip(other.0.iter())
        //                .map(|(b0, b1)| b0 - b1)
        //                .collect(),
        //        )
    }
}
impl ops::Mul for Bytes32 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let a = BigUint::from_bytes_be(self.0.as_slice());

        let b = BigUint::from_bytes_be(other.0.as_slice());

        let sum = a * b;
        Bytes32(BigUint::to_bytes_be(&sum))

        //        Bytes32(
        //            self.0
        //                .iter()
        //                .zip(other.0.iter())
        //                .map(|(b0, b1)| b0 - b1)
        //                .collect(),
        //        )
    }
}

impl ops::Div for Bytes32 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let a = BigUint::from_bytes_be(self.0.as_slice());

        let b = BigUint::from_bytes_be(other.0.as_slice());

        let sum = a / b;
        Bytes32(BigUint::to_bytes_be(&sum))

        //        Bytes32(
        //            self.0
        //                .iter()
        //                .zip(other.0.iter())
        //                .map(|(b0, b1)| b0 & b1)
        //                .collect(),
        //        )
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

impl ops::BitOr for Bytes32 {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Bytes32(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(b0, b1)| b0 | b1)
                .collect(),
        )
    }
}

impl ops::BitXor for Bytes32 {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Bytes32(
            self.0
                .iter()
                .zip(rhs.0.iter())
                .map(|(b0, b1)| b0 ^ b1)
                .collect(),
        )
    }
}

impl ops::Rem for Bytes32 {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        let a = BigUint::from_bytes_be(self.0.as_slice());

        let b = BigUint::from_bytes_be(rhs.0.as_slice());

        let rem = a % b;
        Bytes32(BigUint::to_bytes_be(&rem))
    }
}

impl ops::Not for Bytes32 {
    type Output = Self;

    fn not(self) -> Self::Output {
        let a = BigUint::from_bytes_be(self.0.as_slice());
        Bytes32::new()
    }
}
#[cfg(test)]
mod tests {

    #[test]
    fn test_add() {
        let a = Bytes32::new() ;

        assert_eq!(result, 4);
    }
}
