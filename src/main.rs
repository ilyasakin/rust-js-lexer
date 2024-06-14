use std::io::{Error, ErrorKind};

// Define Token
#[derive(Debug)]
enum EToken {
    Identifier(String),
    Keyword(String),
    Operator(String),
    Punctuator(String),
    Literal(String),
}

#[derive(Debug)]
struct Token {
    token: EToken,
    line_number: u32,
    column_start_number: u32,
    column_end_number: u32,
}

impl Token {
    fn new(
        token: EToken,
        line_number: u32,
        column_start_number: u32,
        column_end_number: u32,
    ) -> Token {
        return Token {
            token,
            line_number,
            column_start_number,
            column_end_number,
        };
    }
}

struct Tokenizer {
    code: String,
    current_line_number: u32,
    current_cursor_location: u32,
}

impl Tokenizer {
    fn new(code: String) -> Tokenizer {
        Tokenizer {
            code,
            current_line_number: 0,
            current_cursor_location: 0,
        }
    }

    fn reset_state(&mut self) {
        self.current_line_number = 0;
        self.current_cursor_location = 0;
    }

    fn get_token(&self, token: &str) -> EToken {
        match token {
            "var" => EToken::Keyword(String::from("var")),
            "let" => EToken::Keyword(String::from("let")),
            "const" => EToken::Keyword(String::from("const")),
            "function" => EToken::Keyword(String::from("function")),
            "=>" => EToken::Operator(String::from("=>")),
            "=" => EToken::Operator(String::from("=")),
            "+" => EToken::Operator(String::from("+")),
            "-" => EToken::Operator(String::from("-")),
            "*" => EToken::Operator(String::from("*")),
            "/" => EToken::Operator(String::from("/")),
            "%" => EToken::Operator(String::from("%")),
            "==" => EToken::Operator(String::from("==")),
            "===" => EToken::Operator(String::from("===")),
            "!=" => EToken::Operator(String::from("!=")),
            "!==" => EToken::Operator(String::from("!==")),
            ">" => EToken::Operator(String::from(">")),
            "<" => EToken::Operator(String::from("<")),
            ">=" => EToken::Operator(String::from(">=")),
            "<=" => EToken::Operator(String::from("<=")),
            "&&" => EToken::Operator(String::from("&&")),
            "||" => EToken::Operator(String::from("||")),
            "!" => EToken::Operator(String::from("!")),
            "&" => EToken::Operator(String::from("&")),
            "|" => EToken::Operator(String::from("|")),
            "^" => EToken::Operator(String::from("^")),
            "~" => EToken::Operator(String::from("~")),
            "<<" => EToken::Operator(String::from("<<")),
            ">>" => EToken::Operator(String::from(">>")),
            ">>>" => EToken::Operator(String::from(">>>")),
            "++" => EToken::Operator(String::from("++")),
            "--" => EToken::Operator(String::from("--")),
            "in" => EToken::Operator(String::from("in")),
            "instanceof" => EToken::Operator(String::from("instanceof")),
            "typeof" => EToken::Operator(String::from("typeof")),
            "void" => EToken::Operator(String::from("void")),
            "delete" => EToken::Operator(String::from("delete")),
            _ => EToken::Identifier(String::from(token)),
        }
    }

    fn get_is_next_char_punctuator(&self, line: &str, cursor_location: u32) -> bool {
        let next_char = line.chars().nth(cursor_location as usize + 1);

        match next_char {
            Some(character) => character == ';',
            None => false,
        }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for (index, line) in self.code.lines().enumerate() {
            let mut current_token: String = String::new();
            self.current_line_number = index as u32;

            for (column_index, character) in line.chars().enumerate() {
                self.current_cursor_location = column_index as u32;

                if !character.is_whitespace()
                    || self.get_is_next_char_punctuator(line, self.current_cursor_location)
                {
                    current_token.push(character);
                    println!("Current Token: {}", current_token);
                } else {
                    let current_token_length = current_token.len() as u32;

                    if !current_token.is_empty() {
                        // TODO: Probably not the best way to circumvent the borrow checker.
                        let built_token_string = current_token.clone();

                        tokens.push(Token::new(
                            self.get_token(&built_token_string),
                            self.current_line_number,
                            self.current_cursor_location - current_token_length,
                            self.current_cursor_location,
                        ));

                        current_token.clear();
                    }
                }
            }
        }

        self.reset_state();

        return tokens;
    }
}

fn main() {
    let contents: String = std::fs::read_to_string("./src/samples/variable_declaration.js")
        .expect("Something went wrong reading the file");

    let tokens: Vec<Token> = Tokenizer::new(contents).tokenize();

    println!("{:?}", tokens);
}
