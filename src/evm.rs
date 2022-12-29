use crate::memory::Memory;
use crate::stack::Stack;
use crate::storage::Storage;

#[allow(dead_code)]
pub struct EVM {
    stack: Stack,
    memory: Memory,
    storage: Storage,
}

impl EVM {
    pub fn new(&self) -> Self {
        EVM {
            stack: Stack::new(),
            memory: Memory::new(),
            storage: Storage::new(),
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
