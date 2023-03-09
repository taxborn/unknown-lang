//! Entrypoint for the Unknown Language Parser

use crate::lexer::{state::Lexer, tokens::Token};

pub struct Parser<'lex> {
    lexer: Lexer<'lex>,
}

impl<'lex> Parser<'lex> {
    pub fn new(lexer: Lexer<'lex>) -> Self {
        Self { lexer }
    }

    pub fn iter_thru_tokens(&mut self) {
        while let Ok(tok) = self.lexer.lex_next() {
            if tok == Token::Eof {
                break;
            }

            println!("token found: {tok:?}");
        }
    }
}
