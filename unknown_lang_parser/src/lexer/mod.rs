pub mod comments;
pub mod literals;
pub mod state;
pub mod tokens;
pub mod errors;

use self::{state::Lexer, tokens::Token, errors::LexingError};

type TokenResult = Result<Token, LexingError>;

/// Checking if a given character is a whitespace character. Currently this
/// this only checks '\r' and '\t', however there is a more exhaustive list
/// in the Rust lexer, which I might be able to update this to base off of.
fn is_whitespace(chr: char) -> bool {
    matches!(chr, ' ' | '\r' | '\t')
}

/// Checking if a given character is valid for identifiers. Currently according
/// to the grammar, this is all letters lowercase and uppercase, numbers, and
/// underscores.
fn is_valid_id(chr: char) -> bool {
    chr.is_alphanumeric() || matches!(chr, '_')
}

/// Checking if a given character is valid for the *start* of identifiers.
/// Currently according to the grammar, this is all letters lowercase and
/// uppercase, and underscores. We do not allow for numbers to be the start
/// because that can cause issues with parsing of actual numbers.
fn is_valid_id_start(chr: char) -> bool {
    chr.is_alphabetic() || matches!(chr, '_')
}

impl<'a> Lexer<'a> {
    /// next_char to the next character and output its token
    pub fn single_token(&mut self, token: Token) -> Token {
        self.next_char();
        token
    }

    /// Get the next *meaningful* [`Token`] from the lexer. Meaningful here means any token
    /// which is not a comment token.
    pub fn get_next_token(&mut self) -> TokenResult {
        loop {
            let token = self.lex_token()?;

            // If the current token is a comment token, ignore it and continue to the
            // next loops iteration
            if let Token::Comment(_, _) = token {
                continue;
            }

            return Ok(token);
        }
    }

    /// Get the next token from the lexer.
    fn lex_token(&mut self) -> TokenResult {
        // Check if there is a character to move to
        if let Some(&chr) = self.lookahead.peek() {
            match chr {
                c if is_whitespace(c) => {
                    self.accumulate_while(&is_whitespace);
                    self.lex_token()
                }
                '\n' => {
                    self.accumulate_while(&|x| matches!(x, '\n' | '\r'));
                    self.lex_token()
                }
                '.' => {
                    self.next_char();
                    match self.lookahead.peek() {
                        Some('.') => Ok(self.single_token(Token::DotDot)),
                        _ => Ok(Token::Dot),
                    }
                }
                '(' => Ok(self.single_token(Token::LPar)),
                ')' => Ok(self.single_token(Token::RPar)),
                '[' => Ok(self.single_token(Token::LBracket)),
                ']' => Ok(self.single_token(Token::RBracket)),
                '{' => Ok(self.single_token(Token::LBrace)),
                '}' => Ok(self.single_token(Token::RBrace)),
                '=' => {
                    self.next_char();
                    match self.lookahead.peek() {
                        Some('=') => Ok(self.single_token(Token::EqEq)),
                        Some('>') => Ok(self.single_token(Token::FatArrow)),
                        _ => Ok(Token::Eq),
                    }
                }
                ':' => {
                    self.next_char();
                    match self.lookahead.peek() {
                        Some(':') => Ok(self.single_token(Token::ColonColon)),
                        _ => Ok(Token::Colon),
                    }
                }
                ';' => Ok(self.single_token(Token::Semi)),
                '$' => Ok(self.single_token(Token::Dollar)),
                ',' => Ok(self.single_token(Token::Comma)),
                '-' => {
                    self.next_char();
                    match self.lookahead.peek() {
                        Some('>') => Ok(self.single_token(Token::RightArrow)),
                        _ => Ok(Token::Minus),
                    }
                }
                '~' => Ok(self.single_token(Token::Tilde)),
                '+' => {
                    self.next_char();
                    match self.lookahead.peek() {
                        Some('=') => Ok(self.single_token(Token::PlusEq)),
                        _ => Ok(Token::Plus),
                    }
                }
                '*' => Ok(self.single_token(Token::Star)),
                '/' => {
                    self.next_char();
                    match self.lookahead.peek() {
                        // Single-line comment
                        Some('/') => self.lex_comment(),
                        // Multi-line comment
                        Some('*') => self.lex_multiline_comment(),
                        _ => Ok(Token::Slash),
                    }
                }
                '%' => Ok(self.single_token(Token::Percent)),
                '&' => Ok(self.single_token(Token::Ampersand)),
                '|' => Ok(self.single_token(Token::Bar)),
                '^' => Ok(self.single_token(Token::Hat)),
                '>' => {
                    self.next_char();
                    match self.lookahead.peek() {
                        Some('=') => Ok(self.single_token(Token::GreaterEq)),
                        Some('>') => Ok(self.single_token(Token::GreaterGreater)),
                        _ => Ok(Token::Greater),
                    }
                }
                '<' => {
                    self.next_char();
                    match self.lookahead.peek() {
                        Some('=') => Ok(self.single_token(Token::LessEq)),
                        Some('<') => Ok(self.single_token(Token::LessLess)),
                        _ => Ok(Token::Less),
                    }
                }
                '!' => {
                    self.next_char();
                    match self.lookahead.peek() {
                        Some('=') => Ok(self.single_token(Token::BangEq)),
                        _ => Ok(Token::Bang),
                    }
                }
                '"' => self.lex_string(),
                c if c.is_ascii_digit() => Ok(self.single_token(Token::Bang)),
                // TODO: Numbers
                // TODO: Does the lexer need to be moved to the next character
                // after accumulate_while?
                c if is_valid_id_start(c) => {
                    let ident = self.accumulate_while(&is_valid_id).to_string();
                    Ok(Token::Ident(ident))
                }
                c => Ok(Token::Error(c)),
            }
        } else {
            // If there is no character to move to, return an EOF Token
            Ok(Token::Eof)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to get all the tokens at once within the Lexer
    fn get_toks<'a>(lexer: &mut Lexer) -> Vec<Token> {
        let mut toks: Vec<Token> = vec![];

        loop {
            let tok = lexer.lex_next();

            if tok == Ok(Token::Eof) {
                break;
            }

            toks.push(tok.unwrap());
        }

        toks
    }

    #[test]
    fn test_triples() {
        let input = "<<<>>>";
        let mut lexer = Lexer::new(input);
        let toks = get_toks(&mut lexer);
        let expected = vec![
            Token::LessLess,
            Token::Less,
            Token::GreaterGreater,
            Token::Greater,
        ];
        assert_eq!(toks, expected);

        let exp_eof = lexer.lex_next();
        assert_eq!(exp_eof, Ok(Token::Eof));
    }

    #[test]
    fn test_assignment() {
        let input = "let a := 5;";
        let mut lexer = Lexer::new(input);

        let toks = get_toks(&mut lexer);

        let expected = vec![
            Token::Ident("let".to_string()),
            Token::Ident("a".to_string()),
            Token::Colon,
            Token::Eq,
            // TODO: When numbers are implemented, this will break and need to
            // be updated.
            Token::Bang,
            Token::Semi,
        ];

        assert_eq!(toks, expected);

        let tok = lexer.lex_next();
        assert_eq!(tok, Ok(Token::Eof));
    }
}
