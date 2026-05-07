use std::iter;

use crate::lexer::token::Token;

pub mod token;

pub fn lex(_: &str) -> impl Iterator<Item = Token> + '_ {
    iter::from_fn(move || None)
}
