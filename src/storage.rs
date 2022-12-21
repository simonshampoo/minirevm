use core::fmt;
use std::collections::HashMap;

pub type Bytes32 = [u8; 4];
pub struct Storage {
    pub kvstore: HashMap<Bytes32, Bytes32>,
}
