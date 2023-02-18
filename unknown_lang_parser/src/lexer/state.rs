use std::{iter::Peekable, str::CharIndices};

#[derive(Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    iter: Peekable<CharIndices<'a>>,
    c: char,
    ci: usize,
    error: bool,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_char();

        if self.c == '\x00' {
            return None;
        }

        Some(self.c)
    }
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

    fn next_char(&mut self) {
        if let Some((index, chr)) = self.iter.next() {
            self.ci = index;
            self.c = chr;
        } else {
            // If we are at the end,
            self.ci = self.input.len();
            self.c = '\x00';
        }
    }
}
