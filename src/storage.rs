use std::collections::HashMap;

pub type Bytes32 = [u8; 4];
pub struct Storage {
    pub kvstore: HashMap::<Bytes32, Bytes32>,
    pub gas: u16,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            kvstore: HashMap::<Bytes32, Bytes32>::new(),
            gas: 0,
        }
    }

    pub fn sstore(&mut self, key: Bytes32, value: Bytes32) {
        self.kvstore.insert(key, value);
        self.gas += 0; //TODO 

    }

    pub fn sload(&self, key: Bytes32) -> Option<Bytes32>{
        let val = self.kvstore.get(&key);
        
        match val {
            Some(val) => Some(*val), // what the fuck
            _  =>  None
        }
    }

    pub fn get_gas(&self) -> u16 {
        self.gas
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
