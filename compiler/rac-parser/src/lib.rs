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
pub mod tokeniter;

use std::iter;
use crate::{lexer::Lexer, token::{Token, TokenKind}};
