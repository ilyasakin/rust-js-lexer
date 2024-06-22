use crate::token::{EToken, Token};

pub struct Tokenizer {
    code: String,
    current_line_number: u32,
    current_cursor_location: u32,
}

impl Tokenizer {
    pub fn new(code: String) -> Tokenizer {
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

    fn determine_token(&self, token: &str) -> EToken {
        if token.chars().all(char::is_alphabetic) {
            return EToken::Identifier(String::from(token));
        }

        if token.chars().all(char::is_numeric) {
            return EToken::Literal(String::from(token));
        }

        return EToken::Identifier(String::from(token));
    }

    fn get_token(&self, token: &str) -> EToken {
        println!("Getting Token: {}", token);

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
            ";" => EToken::Punctuator(String::from(";")),
            _ => self.determine_token(token),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = vec![];

        for (index, line) in self.code.lines().enumerate() {
            let mut current_token: String = String::new();
            self.current_line_number = index as u32;

            for (column_index, character) in line.chars().enumerate() {
                self.current_cursor_location = column_index as u32;

                let is_last_character = column_index == line.len() - 1;
                let is_next_char_semicolon =
                    line.chars().nth(column_index + 1).unwrap_or(' ') == ';';

                if !character.is_whitespace() {
                    if is_last_character || is_next_char_semicolon {
                        current_token.push(character);
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
                    } else {
                        current_token.push(character);
                    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_correct_amount_of_tokens() {
        let code = String::from("var x = 10;");
        let mut tokenizer = Tokenizer::new(code);
        let tokens = tokenizer.tokenize();

        assert_eq!(tokens.len(), 5);
    }

    #[test]
    fn generates_correct_tokens() {
        let code = String::from("var x = 10;");
        let mut tokenizer = Tokenizer::new(code);
        let tokens = tokenizer.tokenize();

        let correct_tokens = vec![
            Token::new(EToken::Keyword(String::from("var")), 0, 0, 2),
            Token::new(EToken::Identifier(String::from("x")), 0, 4, 4),
            Token::new(EToken::Operator(String::from("=")), 0, 6, 6),
            Token::new(EToken::Literal(String::from("10")), 0, 8, 9),
            Token::new(EToken::Punctuator(String::from(";")), 0, 9, 9),
        ];

        for (index, token) in tokens.iter().enumerate() {
            let correct_token = &correct_tokens.get(index);
            assert!(correct_token.is_some());
            assert_eq!(token.token, correct_token.unwrap().token);
        }
    }
}
