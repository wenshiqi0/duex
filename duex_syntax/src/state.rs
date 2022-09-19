use std::fmt::Display;

use crate::token::Token;

#[derive(Clone)]
pub struct SyntaxState {
    pub scope_name: String,
    pub token: Token,
    pub depth: usize,

}

impl SyntaxState {
    pub fn new() -> SyntaxState {
        SyntaxState {
            scope_name: "".to_owned(),
            token: Token::Unknown,
            depth: 0,
        }
    }
}

impl Display for SyntaxState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.token {
            Token::Start => write!(f, "Start"),
            Token::Unknown => write!(f, "Unknown"),
            Token::Block => write!(f, "Block"),
            _ => write!(f, "{}::{}", self.token, self.scope_name),
        }
    }
}
