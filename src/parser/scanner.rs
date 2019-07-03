extern crate regex;

pub struct Scanner {}

fn is_alphabetical(s: String) -> bool {
    let alphabetical = regex::Regex::new(r"[A-Za-z]").unwrap();
    alphabetical.is_match(&s)
}

fn is_number(s: String) -> bool {
    let numbers = regex::Regex::new(r"[0-9]").unwrap();
    numbers.is_match(&s)
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {}
    }

    pub fn scan<'a>(&mut self, input: &'a str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars();
        loop {

            match chars.clone().peekable().peek() {
                Some(character) => {
                    if is_number(character.to_string()) {
                        tokens.push(self.parse_number(&mut chars));
                        continue;
                    }
                    if is_alphabetical(character.to_string()) {
                        tokens.push(self.parse_word(&mut chars));
                        continue;
                    }
                    match character {
                        '(' | ')' | '+' | '-' | '*' | '/' => {
                            chars.next();
                            tokens.push(character.to_string())
                        }
                        ' ' | '\n' => {
                            chars.next();
                            continue;
                        }
                        _ => break,
                    }

                }
                None => break,
            }
        }

        tokens
    }

    fn parse_word<'a>(&mut self, chars: &mut std::str::Chars<'a>) -> String {
        let mut word = Vec::<String>::new();
        while let Some(c) = chars.next() {
            if is_alphabetical(c.to_string()) {
                word.push(c.to_string());
                continue;
            } else {
                break;
            }
        }
        word.concat()
    }

    fn parse_number<'a>(&mut self, chars: &mut std::str::Chars<'a>) -> String {
        let mut number = Vec::<String>::new();

        while let Some(c) = chars.next() {
            match c {
                ' ' | '\n' => return number.concat(),
                _ => {
                    if is_number(c.to_string()) {
                        number.push(c.to_string())
                    } else {
                        return number.concat();
                    }
                }
            }
        }
        number.concat()
    }
}
