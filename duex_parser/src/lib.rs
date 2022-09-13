mod ascii;

use std::{cell::RefCell, rc::Rc};

use ascii::ASCII;

use duex_syntax::{node::Node, token::Token};

pub enum Words {
    Normal,
    Ignore,
    Numberic,
    Symbol,
    // Block,
}

pub fn preparse(bytes: &[u8], index: usize) -> Words {
    let character = bytes[index];
    match ascii::from_u8(character) {
        Some(code) => match code {
            // spec keywords
            ASCII::Whitespace | ASCII::Semicolon | ASCII::Newline | ASCII::Return => Words::Ignore,

            // symbol
            ASCII::Equal => Words::Symbol,

            // numberic
            ASCII::Zero
            | ASCII::One
            | ASCII::Two
            | ASCII::Three
            | ASCII::Four
            | ASCII::Five
            | ASCII::Six
            | ASCII::Seven
            | ASCII::Eight
            | ASCII::Nine => Words::Numberic,
        },
        None => Words::Normal,
    }
}

fn parse_after_keyword(keyword: &str) -> Token {
    match keyword {
        "let" => Token::Variable,
        _ => Token::Unknown,
    }
}

pub fn parse(last: &Rc<RefCell<Node>>, word: Words, frag: &str) -> Token {
    let node = last.clone();
    let last_token = node.borrow().get_state().token;
    let last_scope_name = node.borrow().get_state().scope_name;

    // try parse numberic first
    match word {
        Words::Numberic => Token::Numberic,
        _ => match last_token {
            Token::Keyword => parse_after_keyword(&last_scope_name),
            _ => match frag {
                "let" => Token::Keyword,
                "=" => Token::Equal,
                _ => Token::Unknown,
            },
        },
    }
}
