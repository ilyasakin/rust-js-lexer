#[derive(Debug, PartialEq, Eq)]
pub enum EToken {
    Identifier(String),
    Keyword(String),
    Operator(String),
    Punctuator(String),
    Literal(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub token: EToken,
    pub line_number: u32,
    pub column_start_number: u32,
    pub column_end_number: u32,
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
