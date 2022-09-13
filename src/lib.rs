#[cfg(test)]
mod tests {
    use duex_lexer::config::LexerConfig;
    use duex_lexer::LexerBuilder;
    use duex_syntax::node::debug_syntax_tree;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_lexer() {
        let mut file = File::open("example/index.ts").unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let mut lexer = LexerBuilder::build(LexerConfig {
            source: content.to_owned(),
        });

        loop {
            match lexer.next() {
                Some(_) => {}
                None => break,
            }
        }

        debug_syntax_tree(lexer.get_grammar());
    }
}
