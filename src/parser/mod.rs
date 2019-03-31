mod parser;

pub fn parse<'a>(input: &'a str) {
    let tokens = parser::tokenize(input);

    let tokenized = parser::lex(tokens);
    println!("Parsing {:?}", tokenized);
}
