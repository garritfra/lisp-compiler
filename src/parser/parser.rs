extern crate regex;

use regex::Regex;

#[derive(Debug)]
pub struct Token<'a> {
    identifier: Type<'a>,
    value: String,
}

impl<'a> Token<'a> {
    fn new<'b> (identifier: Type<'a>, value: String) -> Self {
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

pub fn tokenize (input: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let specials: Vec<String> = "()+-*/".split("").map(|s| s.to_string()).collect();
    let mut temp_token: Vec<String> = Vec::new();
    let letters: Vec<String> = input.split("").map(|s| s.to_string()).collect();
    for letter in &letters {
        if specials.contains(letter) {
            tokens.push(temp_token.join(""));
            temp_token.clear();
            tokens.push(letter.to_string());
        } else {
            if specials.contains(letter) || letter == &" " {
                tokens.push(temp_token.join(""));
                temp_token.clear();
            } else {
                temp_token.push(letter.to_string());
            }
        }
    }

    tokens.into_iter().filter(|s| s != "").collect()
}

pub fn lex<'a>(tokens: Vec<String>) -> Result<Vec<Token<'a>>, String> {
    let mut tokenized: Vec<Token> = Vec::new();

    for token in tokens {
        match get_type(token.clone()) {
            Some(t) => tokenized.push(Token::new(t, token)),
            None => return Err(format!("Invalid token: {:?}", token)),
        }
    }

    return Ok(tokenized);
}

fn get_type<'a> (token: String) -> Option<Type<'a>> {
    let types: Vec<Type> = vec![
        Type { name: "whitespace", pattern: Regex::new(r"\s").unwrap() },
        Type { name: "open_paren", pattern: Regex::new(r"[(]").unwrap() },
        Type { name: "close_paren", pattern: Regex::new(r"[)]").unwrap() },
        Type { name: "operator", pattern: Regex::new(r"[\+\-\*/]").unwrap() },
        Type { name: "number", pattern: Regex::new(r"^[0-9\.]+$").unwrap() },
    ];

    for t in types {
        if t.pattern.is_match(&token) {
            return Some(t);
        }
    }

    return None;
    
}