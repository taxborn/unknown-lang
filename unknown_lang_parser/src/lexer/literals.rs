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
        let mut string = String::new();

        println!("initial capacity: {}", string.capacity());

        // Consume the initial quote (")
        self.next_char();

        while let Some(&chr) = self.lookahead.peek() {
            match chr {
                // When we encounter another quote, break from the loop
                '"' => break,
                // When a backslash is encountered, check if it is a valid
                // escape character
                '\\' => {
                    // consume the backslash
                    self.next_char();
                    // lex the escaped character and push the result to the
                    // string of it is an `Ok` type.
                    string.push(self.lex_escaped_char()?);
                    continue;
                }
                chr => string.push(chr),
            }

            self.next_char();
        }

        // At this point we *should* only have a '"' left to loop over, so if it
        // is the closing quote, return the Str token, otherwise return an
        // error.
        if let Some('"') = self.next_char() {
            Ok(Token::Str(string))
        } else {
            Err(LexingError::UnclosedString)
        }
    }

    /// If we encounter a backslash, we want to peek ahead to the next
    /// character. If the character would make it a valid escape character,
    /// return the actual escaped character, rather than just the back slash
    /// and escape code independently.
    fn lex_escaped_char(&mut self) -> Result<char, LexingError> {
        if let Some(&chr) = self.lookahead.peek() {
            // now that we know there is something next, consume the forward
            // slash and match on the escaped character
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
    ///
    /// Current idea: 0(17)182FG1 for a base 17 number.
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
                    Some('(') => {
                        // consume base specifier
                        self.next_char();
                        self.lex_arbitrary_base()
                    }
                    _ => self.lex_number_with_base(10),
                }
            }
            Some('0'..='9' | _) => self.lex_number_with_base(10),
        }
    }

    fn lex_number_with_base(&mut self, base: u8) -> TokenResult {
        // TODO: Allow floats, IEEE 754
        let num =
            self.accumulate_while(&|x| x.is_digit(base as u32) || x == '_');
        // TODO: Check if next character is also a number. If so, we know that
        // the number provided wasn't correctly formatted for the base. For
        // example, 0b12312 is not valid, and currently would lex as
        // Number(2, 1) and Number(10, 2312). This might be able to be handled
        // in the parser, but I think an early indication here would be helpful.
        let num = if num.is_empty() { "0" } else { num };

        Ok(Token::Number(base, num.to_string()))
    }

    fn lex_arbitrary_base(&mut self) -> TokenResult {
        // TODO: Parse everything between the parenthesis as a
        // number itself, so we can have something like this:
        // 0(0xF)AFED, which would convert to 0(16)AFED.
        //
        // Another thought: If I go this route, what happens if someone does:
        // 0(0(0))?
        let base = self.accumulate_while(&|x| x != ')').to_string();

        if let Some(')') = self.lookahead.peek() {
            // Consume closing base specifier
            self.next_char();

            if let Ok(radix) = base.parse::<u32>() {
                // TODO: Currently this is a limitation in Rust's `.is_digit()`
                // function. If we want to support larger bases, we need to
                // implement out own `.to_digit()` function. For now, base 36 is
                // fine.
                if radix > 36 {
                    return Err(LexingError::BaseTooLarge(radix as u8));
                }

                let num = self.accumulate_while(&|x| x.is_digit(radix));

                return Ok(Token::Number(radix as u8, num.to_string()));
            }

            return Err(LexingError::UnknownBase(base.to_string()));
        }

        Err(LexingError::UnclosedBaseSpecifier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexes_string() {
        let input = "\"this is a test\"";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Str("this is a test".to_string())));

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }

    #[test]
    fn test_unclosed_string() {
        let input = "\"this is a test";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::UnclosedString));
    }

    #[test]
    fn test_newlines_within_strings() {
        let input = "\"this is a \n\n\ntest\n\"";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Str("this is a \n\n\ntest\n".to_string())));

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }

    #[test]
    fn test_character_escapes() {
        let input = r#""this\ris\na\t \\ \ttest\0""#;
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(
            tok,
            Ok(Token::Str("this\ris\na\t \\ \ttest\0".to_string()))
        );
    }

    #[test]
    fn test_allows_quote() {
        let input = r#""this\ris\na\t \" \\ \ttest\0""#;
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(
            tok,
            Ok(Token::Str("this\ris\na\t \" \\ \ttest\0".to_string()))
        );
    }

    #[test]
    fn test_catches_hanging_escape() {
        let input = r#""unknown sequence\"#;
        let mut lexer = Lexer::new(input);
        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::UnusedEscape));
    }

    #[test]
    fn test_catches_unknown_escape() {
        let input = r#""unknown sequence\p""#;
        let mut lexer = Lexer::new(input);
        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::UnknownEscapedCharacter('p')));
    }

    #[test]
    fn test_lexing_numbers() {
        let input = "123";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(10, "123".to_string())));
    }

    // Since we are going to leave managing negative numbers to parsing, we are
    // checking if these lex into two seperate tokens sequentially.
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
    fn test_lexing_other_base() {
        let input = "0b100101";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(2, "100101".to_string())));
    }

    #[test]
    fn test_lexes_multiple_bases() {
        let input = "0b101 0X13F 0O777 0(9)882";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(2, "101".to_string())));
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(16, "13F".to_string())));
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(8, "777".to_string())));
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(9, "882".to_string())));
    }

    #[test]
    fn test_nullary_input() {
        // TODO: Look more into nullary (or 0-ary) numbers? might have cool
        // properties? This essentially would translate to some constant. What
        // if we default it to the types minimum number? Like so:
        //
        // ```ukl
        // let a : usize = 0(0); // a = 0
        // let b : isize = 0(0); // b = -9_223_372_036_854_775_808
        // ```
        //
        // Seems like a cool trick, however I don't really like this 'hidden' or
        // unintutive meaning behind the syntax. Maybe after writing a bit in
        // the language you can start to understand that 0(0) would mean
        // a 0-ary number, but it isn't immediately apparent what that
        // would mean, and especially doesn't scream minumum of the type.
        let input = "0(0)99";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(0, "".to_string())));
    }

    #[test]
    fn test_even_more_invalid_base() {
        let input = "0(-1)**";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::UnknownBase("-1".to_string())));
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Star));
        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Star));
    }

    #[test]
    fn test_awkward_base() {
        let input = "0(17)123";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(17, "123".to_string())));
    }

    #[test]
    fn test_awkward_max_base() {
        let input = "0(36)123";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(36, "123".to_string())));
    }

    #[test]
    fn test_awkward_max_base_gets_all_letters() {
        let input = "0(36)0123456789abcdefghijklmnopqrstuvwxyz";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(
            tok,
            Ok(Token::Number(
                36,
                "0123456789abcdefghijklmnopqrstuvwxyz".to_string()
            ))
        );
    }

    #[test]
    fn test_awkward_max_base_gets_all_letters_case_insensitive() {
        let input = "0(36)0123456789abCdefghijklmNOPqrstuvwxyz";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(
            tok,
            Ok(Token::Number(
                36,
                "0123456789abCdefghijklmNOPqrstuvwxyz".to_string()
            ))
        );
    }

    #[test]
    fn test_invalid_base() {
        let input = "0(128)123";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::BaseTooLarge(128)));

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Number(10, "123".to_string())));
    }

    #[test]
    fn test_unclosed_base_specifier() {
        let input = "0(21;\n+";
        let mut lexer = Lexer::new(input);

        let tok = lexer.lex_next();
        assert_eq!(tok, Err(LexingError::UnclosedBaseSpecifier));

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }
}
