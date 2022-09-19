use std::fmt;

#[derive(Debug, Clone)]
pub enum Token {
    Start,
    Keyword,
    Function,
    Class,

    Block,

    Equal,
    PlusEqual,
    PlusPlus,

    Variable,
    Constant,

    Numberic,
    Global,
    Dot,
    Property,
    Method,

    LeftParenthes,
    RightParenthes,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    String,

    Unknown,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Start => write!(f, "Start"),
            Token::Keyword => write!(f, "Keyword"),
            Token::Function => write!(f, "Function"),
            Token::Class => write!(f, "Class"),
            Token::Block => write!(f, "Block"),
            Token::Equal => write!(f, "Equal"),
            Token::PlusEqual => write!(f, "PlusEqual"),
            Token::PlusPlus => write!(f, "PlusPlus"),
            Token::Variable => write!(f, "Variable"),
            Token::Constant => write!(f, "Constant"),
            Token::Numberic => write!(f, "Numberic"),
            Token::Global => write!(f, "Global"),
            Token::Dot => write!(f, "Dot"),
            Token::Property => write!(f, "Property"),
            Token::Method => write!(f, "Method"),
            Token::LeftParenthes => write!(f, "LeftParenthes"),
            Token::RightParenthes => write!(f, "RightParenthes"),
            Token::LeftBracket => write!(f, "LeftBracket"),
            Token::RightBracket => write!(f, "RightBracket"),
            Token::LeftBrace => write!(f, "LeftBrace"),
            Token::RightBrace => write!(f, "RightBrace"),
            Token::String => write!(f, "String"),
            Token::Unknown => write!(f, "Unknown"),
        }
    }
}
