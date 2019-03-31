extern crate regex;

use regex::Regex;

#[derive(Debug)]
pub struct Token<'a> {
    identifier: Type<'a>,
    value: &'a str,
}

impl<'a> Token<'a> {
    fn new<'b> (identifier: Type<'a>, value: &'a str) -> Self {
        Self {
            identifier,
            value,
        }
    }
}

#[derive(Debug)]
struct Type<'a> {
    name: &'a str,
    pattern: Regex
}

pub fn tokenize<'a> (input: &'a str) -> Vec<&str> {
    let tokens: Vec<&str> = input.split(" ").collect();
    tokens
}

pub fn lex(tokens: Vec<&str>) -> Vec<Token> {
    let mut tokenized: Vec<Token> = Vec::new();

    for token in tokens {
        match get_type(token) {
            Some(t) => tokenized.push(Token::new(t, token)),
            None => println!("Invalid Token: {:?}", token),
        }
    }

    tokenized
}

fn get_type<'a> (token: &str) -> Option<Type<'a>> {
    let types: Vec<Type> = vec![
        Type { name: "open_paren", pattern: Regex::new(r"[(]").unwrap() },
        Type { name: "close_paren", pattern: Regex::new(r"[)]").unwrap() },
        Type { name: "number", pattern: Regex::new(r"[0-9]*").unwrap() },
        Type { name: "operator", pattern: Regex::new(r"[\+\-\*/]").unwrap() },
        Type { name: "invalid", pattern: Regex::new(r"[\s\S]").unwrap() }
    ];

    for t in types {
        if t.pattern.is_match(token) {
            return Some(t);
        }
    }

    return None;
    
}