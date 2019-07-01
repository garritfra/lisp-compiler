extern crate regex;

pub struct Scanner {}

fn is_alphabetical(s: String) -> bool {
    let alphabetical = regex::Regex::new(r"[A-Za-z]").unwrap();
    alphabetical.is_match(&s)
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn scan<'a>(&mut self, input: &'a str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars();
        loop {
            let character = chars.next();

            match character {
                Some(character) => {
                    if is_alphabetical(character.to_string()) {
                        tokens.push(self.parse_word(character, &mut chars));
                    }
                    match character {
                        '(' | ')' => tokens.push(character.to_string()),

                        _ => break,
                    }
                }
                None => break,
            }
        }

        tokens
    }

    fn parse_word<'a>(&mut self, character: char, chars: &mut std::str::Chars<'a>) -> String {
        let mut word = Vec::<String>::new();

        word.push(character.to_string());

        loop {
            if let Some(c) = chars.next() {
                match c {
                    ' ' | '\n' => {
                        word.push(c.to_string());
                        break;
                    }
                    _ => word.push(c.to_string()),
                }
            }
        }

        word.concat()
    }
}
