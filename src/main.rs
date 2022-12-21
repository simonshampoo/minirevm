use minirevm::evm::EVM;
use minirevm::parser::Parser;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 || args.len() == 1 {
        panic!("incorrect argument length")
    }

    let bytecode = &args[1];
    let parser = Parser::new();

    parser.parse(bytecode);
    /*


        this program will take in bytecode

        then it will simulate the memory, storage, calldata, and stack

        similar to evm.codes playground

        it will be a light version of the evm binary in geth



        parse bytecode first
        we then put it in some kind of mapping like so
        [
            (PUSH, Optional<32>)
            (PUSH, Optional<0>)
            (MSTORE, Optional<_>)
            when we unwrap, we need to make sure that there are two conditions
            1. the opcode does not take any params
            2. the opcode must take stuff off the stack
            3. the params are valid
            ....


        ]
        When we read an operation, we will know when to manipulate memory, storage, etc.
        MSTORE is simple, we simply append to the byte array
        MLOAD is also simple, we just index the stack param

        etc.

    */

    println!("{}", args[1]);
}
