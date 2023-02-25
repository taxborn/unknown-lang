//! Handles lexing single and multiline comments.

use crate::lexer::{state::Lexer, tokens::Token};

impl<'a> Lexer<'a> {
    pub fn lex_single_line_comment(&mut self) -> Token {
        let comment = self.accumulate_while(&|c| c != '\n').to_string();

        // TODO: Same issue as multiline comment below, I would expect that at 
        // this point the `comment` variable would only have the contents of 
        // the comment, not the forward slashes themselves.
        Token::Comment(false, comment[2..].to_string())
    }

    pub fn lex_multiline_comment(&mut self) -> Token {
        let mut closed = false;
        self.advance();
        let startpos = self.ci;

        while let Some((_, chr)) = self.iter.peek() {
            match chr {
                '*' => {
                    self.advance();
                    if let Some((_, '/')) = self.iter.peek() {
                        self.advance();
                        closed = true;
                        // break from the loop since we found the end
                        break;
                    }
                }
                _ => (),
            }
            self.advance();
        }

        // TODO: Replace panics with errors
        if !closed {
            panic!("unclosed multiline comment");
        }

        // TODO: Figure out if there is a better way than shrinking the bounds 
        // by one on each side, this should be intuitive that a multiline 
        // comment's contents should start at the startpos and end where the 
        // current ci is.
        Token::Comment(true, self.input[startpos+1..self.ci-1].to_string())
    }
}
