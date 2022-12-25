use minirevm::parser::Parser;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 || args.len() == 1 {
        panic!("incorrect argument length")
    }

    let bytecode = &args[1];
    let mut parser = Parser::new();

    let instructions = parser.parse(bytecode);

    for instruction in instructions.iter() {
        println!("{:x?}", instruction);
    }
}
