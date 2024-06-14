#[derive(Debug)]
pub enum EToken {
    Identifier(String),
    Keyword(String),
    Operator(String),
    Punctuator(String),
    Literal(String),
}

#[derive(Debug)]
pub struct Token {
    token: EToken,
    line_number: u32,
    column_start_number: u32,
    column_end_number: u32,
}

impl Token {
    pub fn new(
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
