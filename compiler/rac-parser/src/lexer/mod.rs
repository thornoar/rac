use std::{iter, str::CharIndices};

use crate::lexer::token::Token;

mod chars;
pub mod token;

pub struct Lexer<'a> {
    chars: CharIndices<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.char_indices(),
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        todo!()
    }
}

pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lexer = Lexer::new(input);
    iter::from_fn(move || lexer.next_token())
}
