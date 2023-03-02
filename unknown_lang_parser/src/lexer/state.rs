use std::{iter::Peekable, str::Chars};

use super::TokenResult;


#[derive(Debug)]
pub struct Lexer<'a> {
    pub input: &'a str,
    pub lookahead: Peekable<Chars<'a>>,
    pub pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            lookahead: input.chars().peekable(),
            pos: 0,
        }
    }

    pub fn next_char(&mut self) -> Option<char> {
        if let Some(chr) = self.lookahead.next() {
            // since indexing can happen 'within' a character, we want to always
            // increase the index by the length of the character.
            let len = chr.len_utf8();

            // consume the next len characters
            self.input = &self.input[len..];
            // increment the current position
            self.pos += chr.len_utf8();

            return Some(chr);
        }

        // If we are at the end, the index should be the length of the
        // input
        self.pos = self.input.len();

        None
    }

    /// Accumulate while a predicate is true
    pub fn accumulate_while(&mut self, predicate: &dyn Fn(char) -> bool) -> &str {
        let mut size = 0;

        while let Some(&chr) = self.lookahead.peek() {
            // we want to continue looping while the predicate is true, if it
            // is false, we will break from the loop.
            if !predicate(chr) {
                break;
            }

            // increment the size by the utf-8 length, otherwise sometimes we
            // can index 'half-way' into a character, which could have weird
            // consequences.
            size += chr.len_utf8();
            self.lookahead.next();
        }

        // Increase the position
        self.pos += size;
        // Split the input at the specified size
        let (accumulated, rest) = self.input.split_at(size);
        // Consome the accumulated characters
        self.input = rest;
        // Return the output
        accumulated
    }

    pub fn next_chars(&mut self, size: usize) -> Option<&str> {
        // Check to see that the next n characters exist. If any return None,
        // we return None sunce we can't return the next n chars.
        for _ in 0..size {
            if let Some(chr) = self.lookahead.peek() {
                self.pos += chr.len_utf8();
                self.lookahead.next();
            } else {
                return None;
            }
        }

        let str = &self.input[..size];
        self.input = &self.input[size..];
        Some(str)
    }

    #[inline]
    pub fn lex_next(&mut self) -> TokenResult {
        self.lex_token()
    }
}
