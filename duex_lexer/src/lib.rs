pub mod config;
pub mod lexer;

use config::LexerConfig;
use lexer::Lexer;

pub struct LexerBuilder {

}

impl LexerBuilder {
    pub fn build(config: LexerConfig) -> Lexer {
        Lexer::new(config)
    }
}
