mod codegen;
mod parse;
mod tokenize;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        panic!("{}: invalid number of arguments", args[0]);
    }

    let tokens = tokenize::tokenize(&args[1]);

    println!("{:?}", &tokens);

    //let node = parse(tokens);
    //codegen(node);
}
