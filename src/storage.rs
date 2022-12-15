use std::{collections::HashMap, str::Bytes};

#[derive(PartialOrd, PartialEq, Eq)]
struct Bytes32 {
    pub store: Vec<[u8; 4]>,
}

impl core::cmp::Ord for Bytes32 {
    fn cmp(&self, other: &Bytes32) -> std::cmp::Ordering {
        for i in 0..3 {
            if other.store[i] > self.store[i] {
                return std::cmp::Ordering::Greater;
            }
            if other.store[i] < self.store[i] {
                return std::cmp::Ordering::Less;
            }
        }

        std::cmp::Ordering::Equal
    }
}

pub struct Storage {
    kvstore: HashMap<Bytes32, Bytes32>,
}
