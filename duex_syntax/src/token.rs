use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Start,
    Keyword,
    Equal,
    PlusEqual,
    Variable,
    Constant,
    Numberic,
    Global,
    Dot,
    Property,
    Method,
    Unknown,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Start => write!(f, "Start"),
            Token::Keyword => write!(f, "Keyword"),
            Token::Equal => write!(f, "Equal"),
            Token::PlusEqual => write!(f, "PlusEqual"),
            Token::Variable => write!(f, "Variable"),
            Token::Constant => write!(f, "Constant"),
            Token::Numberic => write!(f, "Numberic"),
            Token::Global => write!(f, "Global"),
            Token::Dot => write!(f, "Dot"),
            Token::Property => write!(f, "Property"),
            Token::Method => write!(f, "Method"),
            Token::Unknown => write!(f, "Unknown"),
        }
    }
}
