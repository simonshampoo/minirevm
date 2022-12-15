use std::collections::HashMap;

#[derive(PartialOrd, PartialEq, Eq)]
struct Bytes32 {
    pub store: Vec<[u8; 4]>,
}

impl core::cmp::Ord for Bytes32 {
    fn cmp(&self, other: &Bytes32) -> std::cmp::Ordering {}
}

pub struct Storage {}
