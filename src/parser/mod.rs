mod parser;

pub fn parse<'a>(input: &'a str) -> Result<Vec<parser::Token>, String> {
    let tokens = parser::tokenize(input);
    parser::lex(tokens)
}