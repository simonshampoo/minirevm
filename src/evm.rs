use minirevm::memory;
use minirevm::opcodes;
use minirevm::stack;
use minirevm::storage;
struct EVM {
    stack: Stack,
    memory: Memory,
    storage: Storage,
}
