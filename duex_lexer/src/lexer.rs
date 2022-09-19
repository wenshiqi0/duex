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
    is_func_define: bool,

    index: usize,
    offset: usize,
}

fn set_node_sibling(brother: &Rc<RefCell<Node>>, node: &Rc<RefCell<Node>>) {
    let state = brother.borrow_mut().get_state().clone();
    brother.borrow_mut().set_sibling(&node);
    match brother.borrow().get_parent() {
        Some(parent) => {
            node.borrow_mut().set_weak_parent(parent);
            node.borrow_mut().set_depth(state.depth);
        },
        None => (),
    }
}

fn set_node_parent(parent: &Rc<RefCell<Node>>, node: &Rc<RefCell<Node>>) {
    let state = parent.borrow_mut().get_state().clone();
    parent.borrow_mut().set_first_child(&node);
    node.borrow_mut().set_parent(&parent.clone());
    node.borrow_mut().set_depth(state.depth + 1);
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
            is_func_define: false,
            index: 0,
            offset: 0,
        }
    }

    pub fn get_grammar(&self) -> Rc<RefCell<Node>> {
        self.grammar.clone()
    }

    fn parse_token(&mut self, t: Token, index: usize) -> Option<Token> {
        let source = self.config.source.clone();
        let current = &self.current.clone();
        let current_state = current.borrow().get_state().clone();
        let frag = &source[self.index..index];

        let node = NodeBuilder::build();
        node.borrow_mut().set_state(&t.clone(), frag);

        match t.clone() {
            Token::RightBrace => match current.borrow_mut().get_parent() {
                Some(parent) => {
                    set_node_sibling(&parent.upgrade().unwrap(), &node);
                    self.current = node;
                }
                None => (),
            },
            Token::LeftBrace => {
                let block_node = NodeBuilder::build();
                block_node.borrow_mut().set_state(&Token::Block, "");
                set_node_sibling(current, &node);
                set_node_parent(&node.clone(), &block_node);
                self.current = block_node;
            }
            _ => match current_state.token {
                Token::Start => {
                    set_node_parent(current, &node);
                    self.current = node;
                }
                _ => {
                    set_node_sibling(current, &node);
                    self.current = node;
                }
            },
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
        if do_create_node {
            self.parse_token(token.clone(), index);
        }
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
                            let may_token = self.parse_token_with_ch(ch, index + 1, false);
                            match may_token {
                                Some(t) => match t {
                                    Token::Unknown => {
                                        token = self.parse_token(Token::Method, index + 1)
                                    }
                                    _ => token = self.parse_token_with_ch(ch, index + 1, true),
                                },
                                _ => (),
                            };
                            self.is_after_dot = false;
                        }
                        _ => {
                            if self.is_after_dot {
                                token = self.parse_token(Token::Property, index + 1);
                            } else {
                                token = self.parse_token_with_ch(ch, index + 1, true);
                            }
                            self.is_after_dot = false;
                        }
                    }
                }
                Character::LeftParenthes => {
                    token = self.parse_token(Token::LeftParenthes, index + 1)
                }
                Character::RightParenthes => {
                    token = self.parse_token(Token::RightParenthes, index + 1)
                }
                Character::LeftBracket => token = self.parse_token(Token::LeftBracket, index + 1),
                Character::RightBracket => token = self.parse_token(Token::RightBracket, index + 1),
                Character::LeftBrace => token = self.parse_token(Token::LeftBrace, index + 1),
                Character::RightBrace => token = self.parse_token(Token::RightBrace, index + 1),
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
                            self.parse_token(Token::Dot, index + 1)
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
                Character::Quote | Character::DoubleQuote => {}
                Character::BackQuote => {}
                Character::Unknown => {
                    self.index += 1;
                    self.offset = 0;
                }
            };

            match token.clone() {
                Some(t) => match t {
                    Token::Function => self.is_func_define = true,
                    _ => (),
                },
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
