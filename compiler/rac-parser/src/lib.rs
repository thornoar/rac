//! # RAC Parser
//!
//! This library holds the handwritten recursive-descent parser for Amy with error recovery.
//! Lexical analysis is first performed to produce tokens from a source code string, before that
//! stream of tokens is parsed into an abstract syntax tree for further processing.

#![deny(clippy::pedantic)]
#![deny(unsafe_code)]
// #![deny(warnings)]

pub mod token;
pub mod tokeniter;

use rac_ast::*;
use crate::tokeniter::TokenIter;

pub fn parse<'a> (src: &'a [u8]) -> Module<String> {
    let tokens = TokenIter::new(src, src.len());
    todo!()
}
