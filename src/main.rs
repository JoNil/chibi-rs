mod codegen;
mod parse;
mod tokenize;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        panic!("{}: invalid number of arguments", args[0]);
    }

    let tokens = tokenize(&args[1]);
    let node = parse(tokens);
    codegen(node);

    return 0;
}
