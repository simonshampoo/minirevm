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

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        let result = 2+2;

        assert_eq!(result, 4);
    }

}
