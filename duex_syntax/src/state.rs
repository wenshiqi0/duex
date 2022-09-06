use std::fmt::Display;

use duex_parser::token::Token;

#[derive(Clone)]
pub struct SyntaxState {
    pub scope_name: String,
    pub token: Token,
}

impl SyntaxState {
    pub fn new() -> SyntaxState {
        SyntaxState {
            scope_name: "".to_owned(),
            token: Token::Unknown,
        }
    }
}

impl Display for SyntaxState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token {
            Token::Unknown => write!(f, "Unknown"),
            Token::Equal =>  write!(f, "Equal"),
            Token::Keyword => write!(f, "{}::{}", self.token, self.scope_name),
        }
    }
}
