use crate::types::Bytes32;
use crate::utils::encode_hex;
use std::u64;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    pub memory: Vec<Bytes32>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: Vec::<Bytes32>::new(),
        }
    }

    pub fn mstore(&mut self, offset: Bytes32, val: Bytes32) {
        let o = encode_hex(offset.0.as_slice());
        let off = (u64::from_str_radix(o.as_str(), 16).unwrap() / 32) as usize;
        self.memory[off] = val;
    }

    // idk how this is gonna work with strings and other dynamic memory
    pub fn mload(&self, offset: Bytes32) -> &Bytes32 {
        let o = encode_hex(offset.0.as_slice());
        let off = (u64::from_str_radix(o.as_str(), 16).unwrap() / 32) as usize;
        &self.memory[off] // also this is probably wrong
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;

        assert_eq!(result, 4);
    }
}
