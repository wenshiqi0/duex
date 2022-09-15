use std::{cell::RefCell, rc::Rc};

use duex_parser::{parse, preparse, Character};
use duex_syntax::{node::Node, token::Token, NodeBuilder};

use crate::config::LexerConfig;

pub struct Lexer {
    config: LexerConfig,

    grammar: Rc<RefCell<Node>>,
    current: Rc<RefCell<Node>>,

    done: bool,
    is_after_dot: bool,

    index: usize,
    offset: usize,
}

impl Lexer {
    pub fn new(config: LexerConfig) -> Lexer {
        let grammar = NodeBuilder::init();
        let current = grammar.clone();

        Lexer {
            config,
            grammar,
            current,
            done: false,
            is_after_dot: false,
            index: 0,
            offset: 0,
        }
    }

    pub fn get_grammar(&self) -> Rc<RefCell<Node>> {
        self.grammar.clone()
    }

    fn parse_token(&mut self, t: Token, index: usize, do_create_node: bool) -> Option<Token> {
        let source = self.config.source.clone();
        let current = &self.current;
        let current_state = current.borrow().get_state().clone();
        let frag = &source[self.index..index];

        if do_create_node {
            let node = NodeBuilder::build();
            match current_state.token {
                Token::Start => {
                    current.borrow_mut().set_first_child(&node);
                    node.borrow_mut().set_state(&t, frag);
                    node.borrow_mut()
                        .set_weak_parent(Some(Rc::downgrade(current)));
                }
                _ => {
                    current.borrow_mut().set_sibling(&node);
                    node.borrow_mut().set_state(&t, frag);
                    node.borrow_mut()
                        .set_weak_parent(current.borrow().get_parent());
                }
            };
            self.current = node;
        }

        self.index = index;
        self.offset = 0;
        Some(t.clone())
    }

    fn parse_token_with_ch(
        &mut self,
        ch: Character,
        index: usize,
        do_create_node: bool,
    ) -> Option<Token> {
        let source = self.config.source.clone();
        let current = &self.current;
        let frag = &source[self.index..index];
        let token = parse(current, ch, frag);
        self.parse_token(token.clone(), index, do_create_node);
        Some(token)
    }

    fn parse_dot_dot(&mut self) -> Option<Token> {
        self.index += 2;
        Some(Token::Unknown)
    }

    fn parse_dot_dot_dot(&mut self) -> Option<Token> {
        self.index += 3;
        Some(Token::Unknown)
    }

    fn parse_next(&mut self) -> Option<Token> {
        let mut token: Option<Token> = None;

        if self.done {
            return None;
        }

        while (self.index + self.offset) < self.config.source.len() {
            let source = self.config.source.clone();
            let bytes = source.as_bytes();
            let index = self.index + self.offset;
            let ch = preparse(bytes, index);

            match ch {
                Character::Letter => {
                    let next_ch = preparse(bytes, index + 1);
                    match next_ch {
                        Character::Letter | Character::Numberic => self.offset += 1,
                        Character::LeftParenthes => {
                            token = self.parse_token(Token::Method, index + 1, true);
                            self.is_after_dot = false;
                        }
                        _ => {
                            if self.is_after_dot {
                                token = self.parse_token(Token::Property, index + 1, true);
                            } else {
                                token = self.parse_token_with_ch(ch, index + 1, true);
                            }
                            self.is_after_dot = false;
                        }
                    }
                }
                Character::LeftParenthes => {
                    token = self.parse_token(Token::LeftParenthes, index + 1, true)
                }
                Character::RightParenthes => {
                    token = self.parse_token(Token::RightParenthes, index + 1, true)
                }
                Character::LeftBracket => {
                    token = self.parse_token(Token::LeftBracket, index + 1, true)
                }
                Character::RightBracket => {
                    token = self.parse_token(Token::RightBracket, index + 1, true)
                }
                Character::LeftBrace => token = self.parse_token(Token::LeftBrace, index + 1, true),
                Character::RightBrace => {
                    token = self.parse_token(Token::RightBrace, index + 1, true)
                }
                Character::Ignore => {
                    self.index += 1;
                    self.is_after_dot = false;
                }
                Character::Dot => {
                    let next_ch = preparse(bytes, index + 1);
                    let next_next_ch = preparse(bytes, index + 2);
                    token = match next_ch {
                        Character::Dot => match next_next_ch {
                            Character::Dot => self.parse_dot_dot_dot(),
                            _ => self.parse_dot_dot(),
                        },
                        _ => {
                            self.is_after_dot = true;
                            self.parse_token(Token::Dot, index + 1, true)
                        }
                    };
                }
                Character::Operation => {
                    let next_ch = preparse(bytes, index + 1);
                    match next_ch {
                        Character::Operation => self.offset += 1,
                        _ => token = self.parse_token_with_ch(ch, index + 1, true),
                    }
                    self.is_after_dot = false;
                }
                Character::Numberic => {
                    let next_ch = preparse(bytes, index + 1);
                    match next_ch {
                        Character::Numberic => self.offset += 1,
                        _ => token = self.parse_token_with_ch(ch, index + 1, true),
                    }
                    self.is_after_dot = false;
                }
            };

            match token {
                Some(t) => {
                    token = Some(t);
                    break;
                }
                None => {
                    if index == (bytes.len() - 1) {
                        token = self.parse_token_with_ch(ch, index + 1, false);
                        match token.clone() {
                            None => (),
                            Some(t) => match t {
                                Token::Unknown => token = None,
                                _ => token = self.parse_token_with_ch(ch, index + 1, true),
                            },
                        }
                        // self.index = 0;
                        self.offset = 0;
                        self.done = true;
                    }
                }
            };
        }

        token
    }

    pub fn next(&mut self) -> Option<Token> {
        self.parse_next()
    }
}
