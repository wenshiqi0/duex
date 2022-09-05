pub mod token;
mod ascii;

use token::Token;
use ascii::ASCII;

pub enum Flow {
    Next,
    Break,
}

pub fn flow(bytes: &[u8], index: usize) -> Flow {
    let character = bytes[index];

    match ascii::from_u8(character) {
        Some(code) => {
            match code {
                ASCII::Whitespace => Flow::Break,
                ASCII::Semicolon => Flow::Break,
                ASCII::Newline => Flow::Break,
                ASCII::Return => Flow::Break,
            }
        },
        None => Flow::Next,
    }
}

pub fn parse(frag: &str) -> Token {
    println!("{}", frag);

    match frag {
        "let" => Token::Keyword,
        "=" => Token::Equal,
        _ => Token::Unknown,
    }
}
