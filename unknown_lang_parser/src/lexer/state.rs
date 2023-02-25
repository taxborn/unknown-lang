use crate::lexer::tokens::Token;
use std::{iter::Peekable, str::CharIndices};

use super::{is_valid_id, is_valid_id_start, is_whitespace};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
    c: char,
    ci: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            iter: input.char_indices().peekable(),
            c: '\x00',
            ci: 0,
        }
    }

    fn advance(&mut self) {
        if let Some((index, chr)) = self.iter.next() {
            self.ci = index;
            self.c = chr;
        } else {
            // If we are at the end,
            self.ci = self.input.len();
            self.c = '\x00';
        }
    }

    fn accumulate_while(&mut self, pred: &dyn Fn(char) -> bool) -> &str {
        let start_index = self.ci;

        while let Some((_, chr)) = self.iter.peek() {
            if !pred(*chr) {
                break;
            }

            self.advance();
        }

        &self.input[..=(self.ci - start_index)]
    }

    pub fn lex_token(&mut self) -> Token {
        match self.iter.peek() {
            None => Token::Eof,
            Some((_, chr)) => match chr {
                c if is_whitespace(*c) => {
                    self.accumulate_while(&is_whitespace);
                    self.lex_token()
                }
                '\n' => {
                    self.accumulate_while(&|x| matches!(x, '\n' | '\r'));
                    self.lex_token()
                }
                '.' => {
                    self.advance();
                    match self.iter.peek() {
                        Some((_, '.')) => self.single_token(Token::DotDot),
                        _ => self.single_token(Token::Dot),
                    }
                }
                '(' => self.single_token(Token::LPar),
                ')' => self.single_token(Token::RPar),
                '[' => self.single_token(Token::LBracket),
                ']' => self.single_token(Token::RBracket),
                '{' => self.single_token(Token::LBrace),
                '}' => self.single_token(Token::RBrace),
                '=' => {
                    self.advance();
                    match self.iter.peek() {
                        Some((_, '=')) => self.single_token(Token::EqEq),
                        Some((_, '>')) => self.single_token(Token::FatArrow),
                        _ => self.single_token(Token::Eq),
                    }
                }
                ':' => {
                    self.advance();
                    match self.iter.peek() {
                        Some((_, ':')) => self.single_token(Token::ColonColon),
                        _ => self.single_token(Token::Colon),
                    }
                }
                ';' => self.single_token(Token::Semi),
                '$' => self.single_token(Token::Dollar),
                ',' => self.single_token(Token::Comma),
                '-' => {
                    self.advance();
                    match self.iter.peek() {
                        Some((_, '>')) => self.single_token(Token::RightArrow),
                        _ => self.single_token(Token::Minus),
                    }
                }
                '~' => self.single_token(Token::Tilde),
                '+' => {
                    self.advance();
                    match self.iter.peek() {
                        Some((_, '=')) => self.single_token(Token::PlusEq),
                        _ => self.single_token(Token::Plus),
                    }
                }
                '*' => self.single_token(Token::Star),
                // TODO: comment lexing
                '/' => {
                    self.advance();
                    match self.iter.peek() {
                        // Single-line comment
                        Some((_, '/')) => {
                            let _comment = self.accumulate_while(&|c| c != '\n');

                            println!("comment found: {_comment}");

                            self.lex_token()
                        }
                        // Multi-line comment
                        Some((_, '*')) => {
                            self.lex_multiline_comment();

                            self.lex_token()
                        }
                        _ => self.single_token(Token::Slash),
                    }
                }
                '%' => self.single_token(Token::Percent),
                '&' => self.single_token(Token::Ampersand),
                '|' => self.single_token(Token::Bar),
                '^' => self.single_token(Token::Hat),
                '>' => {
                    self.advance();
                    match self.iter.peek() {
                        Some((_, '=')) => self.single_token(Token::GreaterEq),
                        Some((_, '>')) => self.single_token(Token::GreaterGreater),
                        _ => self.single_token(Token::Greater),
                    }
                }
                '<' => {
                    self.advance();
                    match self.iter.peek() {
                        Some((_, '=')) => self.single_token(Token::LessEq),
                        Some((_, '<')) => self.single_token(Token::LessLess),
                        _ => self.single_token(Token::Less),
                    }
                }
                '!' => {
                    self.advance();
                    match self.iter.peek() {
                        Some((_, '=')) => self.single_token(Token::BangEq),
                        _ => self.single_token(Token::Bang),
                    }
                }
                // TODO: Numbers
                c if c.is_ascii_digit() => Token::Bang,
                // TODO: Does the lexer need to be moved to the next character
                // after accumulate_while?
                c if is_valid_id_start(*c) => {
                    let ident = self.accumulate_while(&is_valid_id).to_string();
                    Token::Ident(ident)
                }
                _ => todo!(),
            },
        }
    }

    /// Helper for the lexer to lex a single token and move to the next
    /// cursor
    fn single_token(&mut self, token: Token) -> Token {
        self.advance();
        token
    }

    fn lex_multiline_comment(&mut self) {
        self.advance();
        let mut closed = false;

        let mut next = |p: &mut Lexer<'a>| {
            p.iter.next();
        };

        while let Some((_, chr)) = self.iter.peek() {
            match chr {
                '*' => {
                    next(self);
                    if let Some((_, '/')) = self.iter.peek() {
                        closed = true;
                        break;
                    }
                }
                _ => (),
            }
            next(self);
        }

        if !closed {
            panic!("unclosed multiline comment");
        } else {
            println!(":)");
        }
    }

    pub fn lex_next_token(&mut self) -> Token {
        self.lex_token()
    }

    fn current_character(&self) -> &char {
        &self.c
    }
}
