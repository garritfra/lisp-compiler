mod parser;


fn main() {

    let input = "(+ 1 2 3)";
    for token in parser::parse(input) {
        println!("{:?}", token)
    }
}
