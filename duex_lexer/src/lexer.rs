use std::{cell::RefCell, rc::Rc};

use duex_parser::{parse, preparse, Words};
use duex_syntax::{node::Node, token::Token, NodeBuilder};

use crate::config::LexerConfig;

pub struct Lexer {
    config: LexerConfig,

    grammar: Rc<RefCell<Node>>,
    current: Rc<RefCell<Node>>,

    done: bool,

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
            index: 0,
            offset: 0,
        }
    }

    pub fn get_grammar(&self) -> Rc<RefCell<Node>> {
        self.grammar.clone()
    }

    fn parse_token(&mut self, word: Words, index: usize) -> Option<Token> {
        let source = self.config.source.clone();
        let current = &self.current;
        let frag = &source[self.index..index];
        let node = NodeBuilder::build();
        let t = parse(current, word, frag);
        current.borrow_mut().set_sibling(&node);
        node.borrow_mut().set_state(&t, frag);
        node.borrow_mut()
            .set_weak_parent(current.borrow().get_parent());
        self.current = node;
        self.index = index;
        self.offset = 0;
        Some(t.clone())
    }

    pub fn next(&mut self) -> Option<Token> {
        let mut token: Option<Token> = None;

        if self.done {
            return None;
        }

        while (self.index + self.offset) < self.config.source.len() {
            let source = self.config.source.clone();
            let bytes = source.as_bytes();
            let index = self.index + self.offset;
            let word = preparse(bytes, index);

            match word {
                Words::Normal => {
                    let next_word = preparse(bytes, index + 1);
                    match next_word {
                        Words::Normal | Words::Numberic => self.offset += 1,
                        _ => token = self.parse_token(word, index + 1),
                    }
                }
                Words::Ignore => self.index += 1,
                Words::Symbol => {
                    let next_word = preparse(bytes, index + 1);
                    match next_word {
                        Words::Symbol => self.offset += 1,
                        _ => token = self.parse_token(word, index + 1),
                    }
                }
                Words::Numberic => {
                    let next_word = preparse(bytes, index + 1);
                    match next_word {
                        Words::Numberic => self.offset += 1,
                        _ => token = self.parse_token(word, index + 1),
                    }
                }
            };

            match token {
                Some(t) => {
                    token = Some(t);
                    break;
                }
                None => {
                    if index == (bytes.len() - 1) {
                        let final_token = self.parse_token(word, index + 1);
                        self.index = 0;
                        self.offset = 0;
                        self.done = true;
                        return final_token;
                    }
                }
            };
        }

        token
    }
}
