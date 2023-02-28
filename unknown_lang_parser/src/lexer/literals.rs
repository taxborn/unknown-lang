use super::{Lexer, Token};

impl<'a> Lexer<'a> {
    pub fn lex_string(&mut self) -> Token {
        let mut string = String::new();
        // TODO: When errors are implemented, change the return type to a
        // [`Result`] and handle then
        let mut error = false;

        let curr = self.next_char().unwrap();

        while let Some(chr) = self.lookahead.peek() {
            match chr {
                _ if chr == &curr => break,
                '\\' => {
                    self.next_char();
                    match self.lex_escaped_char() {
                        None => {
                            let str = self.accumulate_while(&|c| c != curr);
                            error = true;
                            string.push_str(str);
                        }
                        Some(x) => string.push(x),
                    }
                    continue;
                }
                chr => string.push(*chr),
            }

            self.next_char();
        }

        if self.next_char().is_some() {
            return Token::Str(curr == '"', string);
        }

        if error {
            println!("error encountered while parsing character escapes");
        }

        Token::Error('"')
    }

    /// If we encounter a backslash, we want to peek ahead to the next 
    /// character. If the character would make it a valid escape character, 
    /// return the actual escaped character, rather than just the back slash
    /// and escape code independently.
    fn lex_escaped_char(&mut self) -> Option<char> {
        match self.lookahead.peek() {
            // TODO: When errors are implemented, change the return type to a
            // [`Result`] and handle then
            None => None,
            Some(&x) => {
                self.next_char();

                match x {
                    '\'' => Some('\''),
                    '\"' => Some('\"'),
                    'n' => Some('\n'),
                    'r' => Some('\r'),
                    't' => Some('\t'),
                    '0' => Some('\0'),
                    '\\' => Some('\\'),
                    _ => None,
                }
            }
        }
    }

    pub fn lex_number(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_single_quotes() {
        let input = "'this is a test'";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Str(false, "this is a test".to_string()));
    }

    #[test]
    fn test_matching_double_quotes() {
        let input = "\"this is a test\"";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Str(true, "this is a test".to_string()));
    }

    #[test]
    fn test_newlines_within_strings() {
        let input = "'this is a \r\t\ntest'";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Str(false, "this is a \r\t\ntest".to_string()));

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Eof);
    }

    #[test]
    fn test_character_escapes() {
        let input = "\"this is a \ttest\"";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Token::Str(true, "this is a \ttest".to_string()));
    }
}
