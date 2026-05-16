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

use std::collections::VecDeque;

use rac_ast::*;
use rac_diagnostics::Result;
use rac_diagnostics::Result::*;
use rac_diagnostics::Span;
use crate::tokeniter::TokenIter;
use crate::token::Token;
use crate::token::TokenKind as TK;

macro_rules! open_paren {
    ($ts:expr) => {
        let open = $ts.pop();
        if open.kind != TK::OpenParen {
            return Error(open.range, String::from("Expected an opening parenthesis here."));
        }
    };
}

macro_rules! close_paren {
    ($ts:expr) => {
        let open = $ts.pop();
        if open.kind != TK::CloseParen {
            return Error(open.range, String::from("Expected a closing parenthesis here."));
        }
    };
}

fn mkstring<'a> (src: &'a [u8], span: Span) -> Result<String> {
    match str::from_utf8(&src[span.start .. span.end]) {
        Ok(s) => Value(String::from(s)),
        _ => Error(span, String::from("This string does not contain valid ASCII."))
    }
}

type Name = String;

pub fn parse<'a> (src: &'a [u8]) -> Result<Module<Name>> {
    let ts = TokenIter::new(src, src.len());
    parse_module(src, ts)
}

fn parse_module<'a> (src: &'a [u8], mut ts: TokenIter) -> Result<Module<Name>> {
    let t1 = ts.pop();
    if t1.kind != TK::KwObject {
        return Error(t1.range, String::from("A module must start with the keyword `object`."));
    }
    let t2 = ts.pop();
    if t2.kind != TK::Identifier {
        return Error(t2.range, String::from("A module must be given a valid identifier name."));
    }
    
    todo!()
}

fn parse_many_definitions<'a> (src: &'a [u8], mut ts: TokenIter) -> Result<VecDeque<Definition<Name>>> {
    todo!()
}

fn parse_definition<'a> (src: &'a [u8], mut ts: TokenIter) -> Result<Definition<Name>> {
    let kw = ts.pop();
    match kw.kind {
        TK::KwDef => {
            let id = ts.pop();
            if id.kind != TK::Identifier {
                return Error(id.range, String::from("A function must have a valid name."));
            }
            todo!()
        },
        TK::KwAbstract => {
            todo!()
        },
        TK::KwCase => {
            todo!()
        },
        _ => Error(kw.range, String::from("A definition must start with either `def`, `abstract`, or `case`")),
    }
}

// parses `(x: String, y: Int(32), z: Unit)`
fn parse_arglist<'a> (src: &'a [u8], mut ts: TokenIter) -> Result<ArgList<Name>> {
    open_paren!(ts);
    todo!()
}

// parses `x: String, y: Int(32), z: Unit \)` <- with the closing parenthesis
fn parse_many_arguments<'a> (src: &'a [u8], mut ts: TokenIter) -> Result<ArgList<Name>> {
    todo!()
}

// parses `x: String` or `y: Int(32)`
fn parse_argument<'a> (src: &'a [u8], mut ts: TokenIter) -> Result<(Name, Type<Name>)> {
    let id = ts.pop();
    if id.kind != TK::Identifier {
        return Error(id.range, String::from("An argument must have a valid name."));
    }
    mkstring(src, id.range).bind(|s| {
        let colon = ts.pop();
        if colon.kind != TK::Colon {
            return Error(colon.range, String::from("Expected a colon after the argument name."));
        }
        parse_type(src, ts).bind(|t| Value((s, t)))
    })
}

// parses `String` or `Unit` or `Int(32)`
fn parse_type<'a> (src: &'a [u8], mut ts: TokenIter) -> Result<Type<Name>> {
    let typ = ts.pop();
    match typ.kind {
        TK::TypInt => {
            open_paren!(ts);
            let size = ts.pop();
            if size.kind != TK::LitInt {
                return Error(size.range, String::from("Expected an integer literal for the `Int` type."));
            }
            match str::from_utf8(&src[size.range.start .. size.range.end]) {
                Ok("32") => {
                    close_paren!(ts);
                    Value(Type::IntType)
                },
                _ => Error(size.range, String::from("Only size `32` is supported for integer literals."))
            }
        },
        TK::TypBoolean => Value(Type::BoolType),
        TK::TypString => Value(Type::StringType),
        TK::TypUnit => Value(Type::UnitType),
        TK::Identifier => mkstring(src, typ.range).bind(|s| Value(Type::ClassType(s))),
            //
            // match str::from_utf8(&src[typ.range.start .. typ.range.end]) {
            // Ok(name) => Value(Type::ClassType(String::from(name))),
            // _ => Error(typ.range, String::from("Error interpreting this string in ASCII endoding."))
        _ => Error(typ.range, String::from("Expected either a primitive type (`Int`, `Boolean`, `String`, or `Unit`), or an identifier."))
    }
}
