mod tokenizer;

pub fn parse<'a>(input: &'a str) {
    println!("Parsing {:?}", tokenizer::tokenize(input).value);
}
