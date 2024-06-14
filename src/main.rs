mod tokenizer;
mod token;
use crate::token::Token;
use crate::tokenizer::Tokenizer;

fn main() {
    let contents: String = std::fs::read_to_string("./src/samples/variable_declaration.js")
        .expect("Something went wrong reading the file");

    let tokens: Vec<Token> = Tokenizer::new(contents).tokenize();

    println!("{:?}", tokens);
}
