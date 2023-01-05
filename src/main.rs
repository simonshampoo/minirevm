use minirevm::parser::Parser;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("incorrect argument length")
    }

    let bytecode = &args[1];
    let mut parser = Parser::new();

    parser.parse(bytecode);

}
