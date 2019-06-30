extern crate regex;

pub struct Scanner {
    index: i32,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner { index: -1 }
    }

    pub fn scan<'a>(&mut self, input: &'a str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars();

        loop {
            let character = chars.next();
            let alphabetical = regex::Regex::new(r"[A-Za-z]").unwrap();

            match character {
                Some(character) => {
                    if alphabetical.is_match(&character.to_string()) {
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
