use super::{errors::LexingError, Lexer, Token, TokenResult};

// TODO: Evaluate if these are actually needed
#[allow(dead_code)]
impl Token {
    /// Checks if a given token is a binary number
    fn is_bin(&self) -> bool {
        matches!(self, Token::Number(2, _))
    }

    /// Checks if a given token is a octal number
    fn is_oct(&self) -> bool {
        matches!(self, Token::Number(8, _))
    }

    /// Checks if a given token is a decimal number
    fn is_dec(&self) -> bool {
        matches!(self, Token::Number(10, _))
    }

    /// Checks if a given token is a hexadecimal number
    fn is_hex(&self) -> bool {
        matches!(self, Token::Number(16, _))
    }

    /// Checks if a given token is a hexadecimal number
    fn is_base(&self, _radix: u8) -> bool {
        matches!(self, Token::Number(_radix, _))
    }
}

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

        match self.next_char() {
            // If check if we have the another character
            Some(_) => Ok(Token::Str(string)),
            // Otherwise if no character was found, we know that the string did not
            // close properly, so throw an error.
            None => Err(LexingError::UnclosedString),
        }
    }

    /// If we encounter a backslash, we want to peek ahead to the next
    /// character. If the character would make it a valid escape character,
    /// return the actual escaped character, rather than just the back slash
    /// and escape code independently.
    fn lex_escaped_char(&mut self) -> Result<char, LexingError> {
        if let Some(&chr) = self.lookahead.peek() {
            // now that we know there is something next, consume the forward slash and
            // match on the escaped character
            self.next_char();

            return match chr {
                '\'' => Ok('\''),
                '"' => Ok('"'),
                'n' => Ok('\n'),
                'r' => Ok('\r'),
                't' => Ok('\t'),
                '0' => Ok('\0'),
                '\\' => Ok('\\'),
                _ => Err(LexingError::UnknownEscapedCharacter(chr)),
            };
        }

        Err(LexingError::UnusedEscape)
    }

    /// Lex a number. This handles cases where the base is specified, like 
    /// `0b110101` would translate to a binary number rather than a base 10
    /// number. Currently supports bin, oct, dec, and hex, but would be easy
    /// to add further and more abstract bases if needed.
    ///
    /// TODO: Maybe allow for arbitrary bases. Currently Rust's `.is_digit()`
    /// function which is used in the `lex_number_with_base()` helper function
    /// only allows for radicies up to 36.
    pub fn lex_number(&mut self) -> TokenResult {
        // Right now, we are either at the sign character or the first number
        match self.lookahead.peek() {
            // TODO: Should this be an error or EOF token?
            None => Ok(Token::Eof),
            Some('0') => {
                self.next_char();
                // Check if we are converting to another base
                match self.lookahead.peek() {
                    Some('x' | 'X') => {
                        // consume base specifier
                        self.next_char();
                        self.lex_number_with_base(16)
                    }
                    Some('b' | 'B') => {
                        // consume base specifier
                        self.next_char();
                        self.lex_number_with_base(2)
                    }
                    Some('o' | 'O') => {
                        // consume base specifier
                        self.next_char();
                        self.lex_number_with_base(8)
                    }
                    Some('0'..='9' | _) | None => self.lex_number_with_base(10),
                }
            }
            Some('0'..='9' | _) => self.lex_number_with_base(10),
        }
    }

    fn lex_number_with_base(&mut self, base: u8) -> TokenResult {
        // TODO: Allow floats, IEEE 754
        let num = self.accumulate_while(&|x| x.is_digit(base as u32) || x == '_');
        // TODO: Check if next character is also a number. If so, we know that
        // the number provided wasn't correctly formatted for the base. For
        // example, 0b12312 is not valid, and currently would lex as
        // Number(2, 1) and Number(10, 2312). This might be able to be handled
        // in the parser, but I think an early indication here would be helpful.
        let num = if num.is_empty() { "0" } else { num };
        let num = num.to_string();

        Ok(Token::Number(base, num))
    }
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
        assert_eq!(tok, Err(LexingError::UnclosedString));
    }

    #[test]
    fn test_lexing_negative_numbers() {
        let input = "-123";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Minus));
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(10, "123".to_string())));
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }

    #[test]
    fn test_lexing_numbers() {
        let input = "123";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(10, "123".to_string())));
    }

    #[test]
    fn test_lexing_other_base() {
        let input = "0b100101";
        let mut lexer = Lexer::new(input);
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(2, "100101".to_string())));
    }

    #[test]
    fn test_catches_unknown_escape() {
        let input = r#""unknown sequence\p""#;
        let mut lexer = Lexer::new(input);
        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::UnknownEscapedCharacter('p')));
    }

    #[test]
    fn test_catches_hanging_escape() {
        let input = r#""unknown sequence\"#;
        let mut lexer = Lexer::new(input);
        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::UnusedEscape));
    }
}
