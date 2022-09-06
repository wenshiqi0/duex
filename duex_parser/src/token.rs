use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Keyword,
    Equal,
    Unknown,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Keyword => write!(f, "Keyword"),
            Token::Equal => write!(f, "Equal"),
            Token::Unknown => write!(f, "Unknown token!"),
        }
    }
}
