# MiniRevm

mini EVM built in Rust that parses bytecode into a mnemonic and simulates, stack, storage, and memory.

inspired by evm.codes

# Usage

```cargo run <bytecode>```

# Example

`cargo run 60FF61FFFF62FFFFFF63FFFFFFFF6FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF`

Output:

```
=================================================================================
Bytecode received: 600660070200
=================================================================================
0x60 06                                                   (1 bytes)
0x60 07                                                   (1 bytes)
0x2
0x0
=================================================================================
PUSH1 0x06
PUSH1 0x07
### END OF EXECUTION ###
Stack 2a

Memory { memory: [] }
Storage { kvstore: {} }
Stack 2a

Memory { memory: [] }
Storage { kvstore: {} }
=================================================================================
```

# TODO

this project is not finished yet. i am not responsible for any losses you incur using this software. feel free to make a pr though i could use some extra hands

## Urgent
- [ ] left padding for all ```Bytes32``` types 
- [ ] rework ```Memory```

## Not urgent
- [ ] multiple contract support (```.sol``` and ABI)
- [ ] multithreading
- [ ] refine to be more of a library with a cli tool 
