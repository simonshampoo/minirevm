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
Bytecode received: 60FF61FFFF62FFFFFF63FFFFFFFF6FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF
=================================================================================
0x60 FF                                                                (1 bytes)
0x61 FFFF                                                              (2 bytes)
0x62 FFFFFF                                                            (3 bytes)
0x63 FFFFFFFF                                                          (4 bytes)
0x6f FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF                                  (16 bytes)
=================================================================================
```

# TODO

this project is not finished yet. i am not responsible for any losses you incur using this software. feel free to make a pr though i could use some extra hands

[ ] multiple contract support (```.sol``` and ABI)
[ ] multithreading
[ ] refine to be more of a library with a cli tool 
