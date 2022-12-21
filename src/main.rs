use minirevm::evm::EVM;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{}", args.len());

    if args.len() > 2 || args.len() == 1 {
        panic!("incorrect argument length")
    }

    let bytecode = &args[1];

    /*


        this program will take in bytecode

        then it will simulate the memory, storage, calldata, and stack

        similar to evm.codes playground

        it will be a light version of the evm binary in geth



    */

    println!("{}", args[1]);
}
