use duex_parser::{flow, parse, token, Flow};

use crate::config::LexerConfig;

pub struct Lexer {
    config: LexerConfig,

    done: bool,

    index: usize,
    offset: usize,
}

impl Lexer {
    pub fn new(config: LexerConfig) -> Lexer {
        Lexer {
            config,
            done: false,
            index: 0,
            offset: 0,
        }
    }

    pub fn next(&mut self) -> Option<token::Token> {
        let mut token: Option<token::Token> = None;

        if self.done {
            return None;
        }

        while (self.index + self.offset) < self.config.source.len() {
            let source = self.config.source.clone();
            let bytes = source.as_bytes();
            let index = self.index + self.offset;

            match flow(bytes, index) {
                Flow::Next => self.offset += 1,
                Flow::Break => {
                    if self.offset == 0 {
                        self.index += 1;
                    } else {
                        let frag = &source[self.index..index];
                        token = Some(parse(frag));
                        self.index = index + 1;
                        self.offset = 0;
                    }
                }
            };

            match token {
                Some(t) => {
                    token = Some(t);
                    break;
                }
                None =>  {
                    if index == (bytes.len() - 1) {
                        let frag = &source[self.index..(index + 1)];
                        token = Some(parse(frag));
                        self.index = 0;
                        self.offset = 0;
                        self.done = true;
                    }
                }
            };
        }

        token
    }
}
