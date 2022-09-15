mod ascii;

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use ascii::ASCII;

use duex_syntax::{node::Node, token::Token};

#[derive(Copy, Clone)]
pub enum Character {
    Letter,
    Ignore,
    Numberic,
    Operation,
    Dot,

    LeftParenthes,
    RightParenthes,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    // Block,

    Quote,
    DoubleQuote,
    BackQuote,

    Unknown,
}

pub fn preparse(bytes: &[u8], index: usize) -> Character {
    let character = bytes[index];
    match ascii::from_u8(character) {
        Some(code) => match code {
            // spec keywords
            ASCII::Whitespace | ASCII::Semicolon | ASCII::Newline | ASCII::Return => {
                Character::Ignore
            }

            // symbol
            ASCII::Equal => Character::Operation,
            ASCII::Plus => Character::Operation,

            // dot
            ASCII::Dot => Character::Dot,

            // 括号
            ASCII::LeftParenthes => Character::LeftParenthes,
            ASCII::RightParenthes => Character::RightParenthes,
            ASCII::LeftBrace => Character::LeftBrace,
            ASCII::RightBrace => Character::RightBrace,
            ASCII::LeftBracket => Character::LeftBracket,
            ASCII::RightBracket => Character::RightBracket,

            ASCII::Underscore
            | ASCII::LowA
            | ASCII::LowB
            | ASCII::LowC
            | ASCII::LowD
            | ASCII::LowE
            | ASCII::LowF
            | ASCII::LowG
            | ASCII::LowH
            | ASCII::LowI
            | ASCII::LowJ
            | ASCII::LowK
            | ASCII::LowL
            | ASCII::LowM
            | ASCII::LowN
            | ASCII::LowO
            | ASCII::LowP
            | ASCII::LowQ
            | ASCII::LowR
            | ASCII::LowS
            | ASCII::LowT
            | ASCII::LowU
            | ASCII::LowV
            | ASCII::LowW
            | ASCII::LowX
            | ASCII::LowY
            | ASCII::LowZ
            | ASCII::CapA
            | ASCII::CapB
            | ASCII::CapC
            | ASCII::CapD
            | ASCII::CapE
            | ASCII::CapF
            | ASCII::CapG
            | ASCII::CapH
            | ASCII::CapI
            | ASCII::CapJ
            | ASCII::CapK
            | ASCII::CapL
            | ASCII::CapM
            | ASCII::CapN
            | ASCII::CapO
            | ASCII::CapP
            | ASCII::CapQ
            | ASCII::CapR
            | ASCII::CapS
            | ASCII::CapT
            | ASCII::CapU
            | ASCII::CapV
            | ASCII::CapW
            | ASCII::CapX
            | ASCII::CapY
            | ASCII::CapZ => Character::Letter,

            ASCII::Quote => Character::Quote,
            ASCII::DoubleQuote => Character::DoubleQuote,
            ASCII::BackQuote => Character::BackQuote,

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
            | ASCII::Nine => match preparse(bytes, index - 1) {
                Character::Letter => Character::Letter,
                _ => Character::Numberic,
            },
        },
        None => Character::Unknown,
    }
}

fn parse_after_keyword(keyword: &str) -> Token {
    match keyword {
        "let" => Token::Variable,
        "const" => Token::Constant,
        _ => Token::Unknown,
    }
}

fn scroll(child: &Rc<RefCell<Node>>, scope_name: &str) -> Token {
    let state = child.borrow().get_state().clone();

    if state.scope_name == scope_name {
        return state.token.clone();
    }
    match child.borrow().get_sibling().clone() {
        Some(sibling) => scroll(&sibling, scope_name),
        None => Token::Unknown,
    }
}

fn fallback(target: &Weak<RefCell<Node>>, scope_name: &str) -> Token {
    match target.clone().upgrade() {
        Some(parent) => {
            let mut token;

            token = match parent.borrow().get_first_child() {
                Some(child) => scroll(&child, scope_name),
                None => Token::Unknown,
            };

            token = match token {
                Token::Unknown => match &parent.borrow().get_parent().clone() {
                    Some(parent) => fallback(parent, scope_name),
                    None => Token::Unknown,
                },
                _ => token,
            };

            token
        }
        None => Token::Unknown,
    }
}

fn try_global(scope_name: &str) -> Token {
    match scope_name {
        "console" => Token::Global,
        _ => Token::Unknown,
    }
}

fn try_find_scope_name(last: &Rc<RefCell<Node>>, scope_name: &str) -> Token {
    let mut token = Token::Unknown;
    let state = last.borrow().get_state();

    match state.token {
        Token::Variable => {
            if state.scope_name == scope_name {
                return Token::Variable;
            }
        }
        _ => (),
    };

    match last.borrow().get_parent() {
        Some(parent) => {
            token = fallback(&parent.clone(), scope_name);
        }
        _ => (),
    };

    match token {
        Token::Unknown => try_global(scope_name),
        _ => token,
    }
}

pub fn parse(last: &Rc<RefCell<Node>>, character: Character, frag: &str) -> Token {
    let node = last.clone();
    let last_token = node.borrow().get_state().token;
    let last_scope_name = node.borrow().get_state().scope_name;

    match character {
        // try parse numberic first
        Character::Numberic => Token::Numberic,
        _ => match last_token {
            Token::Keyword => parse_after_keyword(&last_scope_name),
            _ => match frag {
                "let" | "const" => Token::Keyword,
                "=" => Token::Equal,
                "+=" => Token::PlusEqual,
                "++" => Token::PlusPlus,
                _ => try_find_scope_name(last, frag.clone()),
            },
        },
    }
}
