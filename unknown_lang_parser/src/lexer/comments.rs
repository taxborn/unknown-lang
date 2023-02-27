//! Handles lexing single and multiline comments.

use crate::lexer::{state::Lexer, tokens::Token};

impl<'a> Lexer<'a> {
    pub fn lex_comment(&mut self) -> Token {
        self.next_char();

        let comment = self.accumulate_while(&|c| c != '\n').to_string();

        // TODO: Same issue as multiline comment below, I would expect that at
        // this point the `comment` variable would only have the contents of
        // the comment, not the forward slashes themselves.
        Token::Comment(false, comment)
    }

    pub fn lex_multiline_comment(&mut self) -> Token {
        let mut closed = false;
        let mut size = 0;
        self.next_char();

        let mut next = |p: &mut Lexer<'a>, chr: char| {
            size += chr.len_utf8();
            p.lookahead.next();
        };

        while let Some(&chr) = self.lookahead.peek() {
            if chr == '*' {
                next(self, chr);
                if let Some('/') = self.lookahead.peek() {
                    closed = true;
                    next(self, chr);
                    // break from the loop since we found the end
                    break;
                }
            }
            next(self, chr);
        }

        self.pos += size;

        // TODO: Replace panics with errors
        if !closed {
            panic!("unclosed multiline comment");
        }

        let comment = &self.input[..size - 2];
        self.input = &self.input[size..];
        Token::Comment(true, comment.to_string())
    }
}

impl Token {
    #[allow(dead_code)]
    fn is_multiline_comment(&self) -> bool {
        matches!(self, Token::Comment(true, _))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexing_single_line_comments() {
        let input = "// This is a comment";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Comment(false, " This is a comment".to_string()));
    }

    #[test]
    fn test_single_line_comment_token_contents() {
        let input = "//test";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Comment(false, "test".to_string()));

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Eof);
    }

    #[test]
    fn test_lexing_multi_line_comments() {
        let input = "/*this is \n\na\n\nmultiline comment*/";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert!(matches!(tok, Token::Comment(true, _)));

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Eof);
    }
}
