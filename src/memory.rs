use crate::types::Bytes32;
use crate::utils::encode_hex;
use std::u64;
use std::u8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Memory {
    pub memory: Vec<u8>,
}

impl Memory {
    pub fn new() -> Self {
        Memory {
            memory: Vec::<u8>::new(),
        }
    }

    pub fn mstore(&mut self, offset: &str, val: Bytes32) {
        let off = u64::from_str_radix(offset, 16).unwrap() as usize;
        if off >= self.memory.len() {
            vec![self.memory, val.0].concat();
        } else {
            self.memory.splice(off..(off+val.0.len()), val.0);
        }
    }

    pub fn mload(&self, offset: Bytes32) -> &[u8] {
        let o = encode_hex(offset.0.as_slice());
        let off = u64::from_str_radix(o.as_str(), 16).unwrap() as usize;
        &self.memory[off..off+32] // also this is probably wrong
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;

        assert_eq!(result, 4);
    }

    #[test]
    fn add_to_memory() {
        let mem = Memory::new();
        for i in 0..1000 {
            mem.memory.push(i); 
        }
        assert_eq!(mem.memory.len(), 1000);
    }

}
