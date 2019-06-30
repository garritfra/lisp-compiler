mod parser;
pub mod scanner;

pub fn parse<'a>(input: &'a str) -> Vec<String> {
    scanner::Scanner::new().scan(input)
}
