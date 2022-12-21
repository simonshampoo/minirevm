use crate::memory::Memory;
use crate::stack::Stack;
use crate::storage::Storage;
pub struct EVM {
    stack: Stack,
    memory: Memory,
    storage: Storage,
}
