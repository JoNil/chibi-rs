mod codegen;
mod parser;
mod tokenizer;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        panic!("{}: invalid number of arguments", args[0]);
    }

    let tokens = tokenizer::tokenize(&args[1]);

    println!("{:?}", &tokens);

    let node = parser::parse(&tokens);

    println!("{:?}", &node);

    //codegen(node);
}
