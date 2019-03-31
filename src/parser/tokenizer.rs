pub struct Token<'a> {
    pub identifier: &'a str,
    pub value: &'a str,
}

impl<'a> Token<'a> {
    fn new<'b> (identifier: &'a str, value: &'a str) -> Self {
        Self {
            identifier,
            value,
        }
    }
}

pub fn tokenize<'a> (input: &'a str) -> Token {
    Token::new("number", input)
}