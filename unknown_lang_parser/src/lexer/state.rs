use std::{iter::Peekable, str::CharIndices};
#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
    c: char,
    ci: usize,
    error: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Self {
            input,
            iter: input.char_indices().peekable(),
            c: '\x00',
            ci: 0,
            error: false,
        };

        // Load the first character of the input into the lexer
        lexer.next_char();

        lexer
    }

    pub fn next_char(&mut self) {
        if let Some((index, chr)) = self.iter.next() {
            self.ci = index;
            self.c = chr;
        } else {
            // If we are at the end,
            self.ci = self.input.len();
            self.c = '\x00';
        }
    }

    pub fn accumulate_while(&mut self, pred: &dyn Fn(char) -> bool) -> &str {
        let start_index = self.ci;

        while let Some((_, chr)) = self.iter.peek() {
            if !pred(*chr) {
                break;
            }

            self.next_char();
        }

        &self.input[..=(self.ci - start_index)]
    }
}
