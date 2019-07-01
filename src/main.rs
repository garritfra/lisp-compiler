mod parser;

fn main() {
    let input = "(add 1 22 3 )";
    for token in parser::parse(input) {
        println!("{:?}", token)
    }
}
