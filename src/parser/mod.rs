mod parser;

pub fn parse<'a>(input: &'a str) -> Vec<parser::Token>{
    let tokens = parser::tokenize(input);
    parser::lex(tokens)
}
