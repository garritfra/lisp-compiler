mod parser;

pub fn parse<'a>(input: &'a str) -> Result<Vec<parser::Token>, String> {
    let tokens = parser::tokenize(input);

    match parser::lex(tokens) {
        Ok(v) => Ok(v),
        Err(e) => Err(e),
    }
}
