# MiniRevm

mini EVM built in Rust that parses bytecode into a mnemonic and simulates, stack, storage, and memory.

inspired by evm.codes

# Usage

cargo run <bytecode>

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
