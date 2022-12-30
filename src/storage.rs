use std::collections::HashMap;

pub type Bytes32 = [u8; 4];
pub struct Storage {
    pub kvstore: HashMap::<Bytes32, Bytes32>,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            kvstore: HashMap::<Bytes32, Bytes32>::new(),
        }
    }

    pub fn sstore(&mut self, key: Bytes32, value: Bytes32) {
        self.kvstore.insert(key, value);
    }

    // this seems so sus... why am I returning an Optional<&_>?
    // val references the value in the hashmap
    // val gets consumed in match... but we return it with Some(val) so we keep the reference. 
    // can we dereference it? what happens to the original map?
    pub fn sload(&self, key: Bytes32) -> Option<&Bytes32> {
        let val = self.kvstore.get(&key);
        
        match val {
            Some(val) => Some(val), 
            _  =>  None
        }
    }

    //pub fn get_gas(&self) -> u16 {
    //    self.gas
    //}
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2 + 2;

        assert_eq!(result, 4);
    }
}
