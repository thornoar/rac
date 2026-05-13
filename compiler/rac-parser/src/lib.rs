//! # RAC Parser
//!
//! This library holds the handwritten recursive-descent parser for Amy with error recovery.
//! Lexical analysis is first performed to produce tokens from a source code string, before that
//! stream of tokens is parsed into an abstract syntax tree for further processing.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
// #![deny(warnings)]

pub mod lexer;
pub mod token;

use std::iter;
use crate::{lexer::Lexer, token::{Token, TokenKind}};

fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
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
