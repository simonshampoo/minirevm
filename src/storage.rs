use std::collections::HashMap;

pub type Bytes32 = [u8; 4];
pub struct Storage {
    pub kvstore: HashMap<Bytes32, Bytes32>,
    pub gas: u16,
}

impl Storage {
    pub fn new() -> Self {
        Storage {
            kvstore: HashMap::<Bytes32, Bytes32>::new(),
            gas: 0,
        }
    }

    pub fn new_storage(&mut self, key: Bytes32, value: Bytes32) {


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
