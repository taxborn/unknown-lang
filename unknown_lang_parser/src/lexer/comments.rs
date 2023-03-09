//! Handles lexing single and multiline comments.

use crate::lexer::{state::Lexer, tokens::Token};

use super::{errors::LexingError, TokenResult};

impl<'a> Lexer<'a> {
    pub fn lex_comment(&mut self) -> TokenResult {
        // Consume the second slash from '//'
        self.next_char();

        // Accumulate until a newline character is found
        let comment = self.accumulate_while(&|c| c != '\n').to_string();

        Ok(Token::Comment(false, comment))
    }

    pub fn lex_multiline_comment(&mut self) -> TokenResult {
        let mut closed = false;
        let mut size = 0;
        self.next_char();

        let mut advance = |p: &mut Lexer<'a>, chr: char| {
            size += chr.len_utf8();
            p.lookahead.next();
        };

        while let Some(&chr) = self.lookahead.peek() {
            if chr == '*' {
                advance(self, chr);
                if let Some('/') = self.lookahead.peek() {
                    closed = true;
                    advance(self, chr);
                    // break from the loop since we found the end
                    break;
                }
            }
            advance(self, chr);
        }

        self.pos += size;

        if !closed {
            return Err(LexingError::UnclosedMutlilineComment);
        }

        // subtract 2 from the size since we don't want to include the last '*/'
        let comment = &self.input[..size - 2];
        self.input = &self.input[size..];
        Ok(Token::Comment(true, comment.to_string()))
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
        let input = "//This is a comment";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(
            tok,
            Ok(Token::Comment(false, "This is a comment".to_string()))
        );

        let tok = tok.unwrap();
        assert!(!tok.is_multiline_comment());

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }

    #[test]
    fn test_comment_retain_spaces() {
        let input = "//  This is a comment";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(
            tok,
            Ok(Token::Comment(false, "  This is a comment".to_string()))
        );
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }

    #[test]
    fn test_lexing_multi_line_comments() {
        let input = r"/* wow!
                       * this is a multiline comment
                       * cool.
                       */";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert!(matches!(tok, Ok(Token::Comment(true, _))));

        let tok = tok.unwrap();
        assert!(tok.is_multiline_comment());

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }

    #[test]
    fn test_unclosed_multiline() {
        let input = r"/* wow!
                       * this is a multiline comment
                       * cool.";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::UnclosedMutlilineComment));
    }
}
