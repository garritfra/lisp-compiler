mod parser;


fn main() {
    let input = "(+ 1 22 3)";
    for token in parser::parse(input).unwrap() {
        println!("{:?}", token)
    }
}
