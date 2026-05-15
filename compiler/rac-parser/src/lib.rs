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
use rac_diagnostics::Result;
use rac_diagnostics::Result::*;
use crate::tokeniter::TokenIter;
use crate::token::Token;
use crate::token::TokenKind as TK;

type Name = String;

pub fn parse<'a> (src: &'a [u8]) -> Result<Module<Name>> {
    let ts = TokenIter::new(src, src.len());
    todo!()
}

fn parse_module<'a> (mut ts: TokenIter) -> Result<Module<Name>> {
    let (t1, t2) = (ts.pop(), ts.pop());
    match (t1.kind, t2.kind) {
        (TK::KwObject, TK::Identifier) => todo!(),
        _ => Error(String::from("")),
    }
}
