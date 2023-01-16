use crate::types::Bytes32;
use evm::Opcode;
use num_bigint::BigUint;
use std::{fmt::Write, num::ParseIntError};

pub fn biguint_to_byte(n: &BigUint) -> Bytes32 {
    Bytes32(BigUint::to_bytes_be(n).to_vec())
}

pub fn decode_hex(s: &str) -> Result<Vec<u64>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u64::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b).unwrap();
    }
    s
}

pub fn match_stackop_n(opcode: Opcode) -> usize {
    match opcode {
        Opcode::PUSH1 | Opcode::SWAP1 | Opcode::DUP1 => 1,
        Opcode::PUSH2 | Opcode::SWAP2 | Opcode::DUP2 => 2,
        Opcode::PUSH3 | Opcode::SWAP3 | Opcode::DUP3 => 3,
        Opcode::PUSH4 | Opcode::SWAP4 | Opcode::DUP4 => 4,
        Opcode::PUSH5 | Opcode::SWAP5 | Opcode::DUP5 => 5,
        Opcode::PUSH6 | Opcode::SWAP6 | Opcode::DUP6 => 6,
        Opcode::PUSH7 | Opcode::SWAP7 | Opcode::DUP7 => 7,
        Opcode::PUSH8 | Opcode::SWAP8 | Opcode::DUP8 => 8,
        Opcode::PUSH9 | Opcode::SWAP9 | Opcode::DUP9 => 9,
        Opcode::PUSH10 | Opcode::SWAP10 | Opcode::DUP10 => 10,
        Opcode::PUSH11 | Opcode::SWAP11 | Opcode::DUP11 => 11,
        Opcode::PUSH12 | Opcode::SWAP12 | Opcode::DUP12 => 12,
        Opcode::PUSH13 | Opcode::SWAP13 | Opcode::DUP13 => 13,
        Opcode::PUSH14 | Opcode::SWAP14 | Opcode::DUP14 => 14,
        Opcode::PUSH15 | Opcode::SWAP15 | Opcode::DUP15 => 15,
        Opcode::PUSH16 | Opcode::SWAP16 | Opcode::DUP16 => 16,
        Opcode::PUSH17 => 17,
        Opcode::PUSH18 => 18,
        Opcode::PUSH19 => 19,
        Opcode::PUSH20 => 20,
        Opcode::PUSH21 => 21,
        Opcode::PUSH22 => 22,
        Opcode::PUSH23 => 23,
        Opcode::PUSH24 => 24,
        Opcode::PUSH25 => 25,
        Opcode::PUSH26 => 26,
        Opcode::PUSH27 => 27,
        Opcode::PUSH28 => 28,
        Opcode::PUSH29 => 29,
        Opcode::PUSH30 => 30,
        Opcode::PUSH31 => 31,
        Opcode::PUSH32 => 32,
        _ => 0,
    }
}
