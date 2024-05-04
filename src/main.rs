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

struct Tokenizer {}

impl Tokenizer {
    fn new() -> Tokenizer {
        Tokenizer {}
    }

    fn tokenize(&self, code: String) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for (index, line) in code.lines().enumerate() {
            println!("{}", line);

            // calculate line number
            let line_number: u32 = (index as u32) + 1;

            // Split line with whitespace
            let words: Vec<&str> = line.split_whitespace().collect();

            for word in words {
                let column_number: u32 = line.find(word).unwrap() as u32;

                // Check if word is a keyword
                if word == "var" || word == "let" || word == "const" {
                    let token: Token = Token::new(
                        EToken::Keyword(String::from(word)),
                        line_number,
                        column_number,
                    );
                    tokens.push(token);
                }

                // Check if word is an identifier
                if word == "x" {
                    let token: Token = Token::new(
                        EToken::Identifier(String::from(word)),
                        line_number,
                        column_number,
                    );
                    tokens.push(token);
                }

                // Check if word is an operator
                if word == "=" || word == "+" || word == "-" || word == "*" || word == "/" {
                    let token: Token = Token::new(
                        EToken::Operator(String::from(word)),
                        line_number,
                        column_number,
                    );
                    tokens.push(token);
                }

                // TODO: This is a simple check, we need to implement a more complex check
                if word == "10" {
                    let token = Token::new(
                        EToken::Literal(String::from(word)),
                        line_number,
                        column_number,
                    );
                    tokens.push(token);
                }
            }
        }

        return tokens;
    }
}

fn main() {
    let contents = std::fs::read_to_string("./src/samples/variable_declaration.js")
        .expect("Something went wrong reading the file");

    let tokens: Vec<Token> = Tokenizer::new().tokenize(contents);

    println!("{:?}", tokens);
}
