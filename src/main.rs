use minirevm::evm::EVM;
use minirevm::parser::Parser;
use minirevm::utils::byte_to_biguint;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("incorrect argument length")
    }

    let bytecode = &args[1];
    let mut parser = Parser::new();

    let instructions = parser.parse(bytecode);

    let mut evm = EVM::new();

    evm.execute_bytecode(instructions);

    evm.print_stack();

    evm.print_memory();

    evm.print_storage();
}
