use super::{errors::LexingError, Lexer, Token, TokenResult};

impl<'a> Lexer<'a> {
    pub fn lex_string(&mut self) -> TokenResult {
        // Consume the initial quote (")
        self.next_char();

        let mut string = String::new();

        while let Some(chr) = self.lookahead.peek() {
            match chr {
                // When we encounter another quote, break from the loop
                '"' => break,
                '\\' => {
                    self.next_char();
                    let escape = self.lex_escaped_char()?;
                    string.push(escape);
                    continue;
                }
                chr => string.push(*chr),
            }

            self.next_char();
        }

        return match self.next_char() {
            // If check if we have the another character
            Some(_) => Ok(Token::Str(string)),
            // Otherwise if no character was found, we know that the string did not
            // close properly, so throw an error.
            None => Err(LexingError::NoNextCharacter)
        }
    }

    /// If we encounter a backslash, we want to peek ahead to the next
    /// character. If the character would make it a valid escape character,
    /// return the actual escaped character, rather than just the back slash
    /// and escape code independently.
    fn lex_escaped_char(&mut self) -> Result<char, LexingError> {
        if let Some(&x) = self.lookahead.peek() {
            self.next_char();
            return match x {
                '\'' => Ok('\''),
                '"' => Ok('"'),
                'n' => Ok('\n'),
                'r' => Ok('\r'),
                't' => Ok('\t'),
                '0' => Ok('\0'),
                '\\' => Ok('\\'),
                _ => Err(LexingError::NoNextCharacter),
            }
        }

        Err(LexingError::NoNextCharacter)
    }

    pub fn lex_number(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matching_double_quotes() {
        let input = "\"this is a test\"";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Str("this is a test".to_string())));
    }

    #[test]
    fn test_newlines_within_strings() {
        let input = "\"this is a \r\t\ntest\"";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Str("this is a \r\t\ntest".to_string())));

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }

    #[test]
    fn test_character_escapes() {
        let input = "\"this is a \ttest\"";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Str("this is a \ttest".to_string())));
    }

    #[test]
    fn test_unclosed_string() {
        let input = "\"this is a test";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::NoNextCharacter));
    }
}
