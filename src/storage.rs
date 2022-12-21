use std::collections::HashMap;

pub type Bytes32 = [u8; 4];
pub struct Storage {
    pub kvstore: HashMap<Bytes32, Bytes32>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            kvstore: HashMap::<Bytes32, Bytes32>::new(),
        }
    }
}
