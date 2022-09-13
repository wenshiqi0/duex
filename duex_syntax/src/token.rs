use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Start,
    Keyword,
    Equal,
    Variable,
    Unknown,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Start => write!(f, "Start"),
            Token::Keyword => write!(f, "Keyword"),
            Token::Equal => write!(f, "Equal"),
            Token::Variable => write!(f, "Variable"),
            Token::Unknown => write!(f, "Unknown"),
        }
    }
}
