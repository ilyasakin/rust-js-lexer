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
    line: u32,
    column: u32,
}

impl Token {
    fn new(token: EToken, line: u32, column: u32) -> Token {
        Token {
            token,
            line,
            column,
        }
    }
}

struct Tokenizer {
    code: String,
    current_line_number: u32,
    current_column_number: u32,
}

impl Tokenizer {
    fn new(code: String) -> Tokenizer {
        Tokenizer {
            current_line_number: 0,
            current_column_number: 0,
            code,
        }
    }

    fn reset_state(&mut self) {
        // Code is not considered as a state.
        self.current_line_number = 0;
        self.current_column_number = 0;
    }

    /// Strategy:
    /// 1. Read the next character
    /// 2. Check if the character is a whitespace
    /// 3. If it is a whitespace, skip it
    fn next_char(
        &self,
        code: &String,
        current_line_number: u32,
        current_column_number: u32,
    ) -> Result<char, Error> {
        let current_line = code.lines().nth((current_line_number - 1) as usize);

        if current_line.is_none() {
            // Current line can not be None. This can only happen if the line number is out of bounds.
            panic!("Current line can not be None");
        }

        let current_line = current_line.unwrap();

        let next_char = current_line.chars().nth(current_column_number as usize);

        if next_char.is_none() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "Next character is None",
            ));
        }

        let next_char = next_char.unwrap();

        if next_char.is_whitespace() {
            return self.next_char(code, current_line_number, current_column_number + 1);
        }

        return Ok(next_char);
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for (index, line) in self.code.lines().enumerate() {
            println!("{}", line);

            self.current_line_number = (index as u32) + 1;

            // let next_char = self.next_char(
            //     &self.code,
            //     self.current_line_number,
            //     self.current_column_number,
            // );
            //
            // if next_char.is_err() {
            //     continue;
            // }
            //
            // let next_char = next_char.unwrap();
            // println!("{}", next_char);

            // Split line with whitespace
            let words: Vec<&str> = line.split_whitespace().collect();

            for word in words {
                self.current_column_number = line.find(word).unwrap() as u32;

                // Check if word is a keyword
                if word == "var" || word == "let" || word == "const" {
                    let token: Token = Token::new(
                        EToken::Keyword(String::from(word)),
                        self.current_line_number,
                        self.current_column_number,
                    );
                    tokens.push(token);
                }

                // Check if word is an identifier
                if word == "x" {
                    let token: Token = Token::new(
                        EToken::Identifier(String::from(word)),
                        self.current_line_number,
                        self.current_column_number,
                    );
                    tokens.push(token);
                }

                // Check if word is an operator
                if word == "=" || word == "+" || word == "-" || word == "*" || word == "/" {
                    let token: Token = Token::new(
                        EToken::Operator(String::from(word)),
                        self.current_line_number,
                        self.current_column_number,
                    );
                    tokens.push(token);
                }

                // TODO: This is a simple check, we need to implement a more complex check
                if word == "10" {
                    let token = Token::new(
                        EToken::Literal(String::from(word)),
                        self.current_line_number,
                        self.current_column_number,
                    );
                    tokens.push(token);
                }
            }
        }

        self.reset_state();

        return tokens;
    }
}

fn main() {
    let contents = std::fs::read_to_string("./src/samples/variable_declaration.js")
        .expect("Something went wrong reading the file");

    let tokens: Vec<Token> = Tokenizer::new(contents).tokenize();

    println!("{:?}", tokens);
}
