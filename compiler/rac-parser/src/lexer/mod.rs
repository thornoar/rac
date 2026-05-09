use std::{iter, str::CharIndices};

use crate::lexer::token::{Token, TokenKind};

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

    pub fn next_token(&mut self) -> Token {
        let Some((_, first)) = self.bump() else {
            return Token::new(TokenKind::Eof, 0..=0);
        };

        match first {
            _ => todo!(),
        }
    }
}

pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lexer = Lexer::new(input);

    iter::from_fn(move || {
        let tok = lexer.next_token();
        if tok.kind != TokenKind::Eof {
            Some(tok)
        } else {
            None
        }
    })
}
