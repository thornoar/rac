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

use std::collections::{VecDeque};
use std::result::Result;

use rac_ast::*;
use rac_diagnostics::{Report, Stage, Span, select};
use crate::tokeniter::TokenIter;
use crate::token::TokenKind as TK;

macro_rules! expect {
    ($ts:expr, $tk:expr, $msg:expr) => {{
        let token = $ts.pop();
        if token.kind != $tk {
            return Err(Report { stage: Stage::Parsing, span: token.range, msg: String::from($msg) });
        }
        token
    }};
}

macro_rules! error {
    ($span:expr, $msg:literal) => {
        Err(Report { stage: Stage::Parsing, span: $span, msg: String::from($msg) })
    };
}

fn mkstring<'a> (src: &'a [u8], span: Span) -> Result<String, Report> {
    match str::from_utf8(&src[span.start .. span.end]) {
        Ok(s) => Ok(String::from(s)),
        _ => error!(span, "This string does not contain valid ASCII.")
    }
}

type Name = String;

pub fn parse<'a> (src: &'a [u8]) -> Result<Module<Name>, Report> {
    let mut ts = TokenIter::new(src, src.len());
    parse_module(src, &mut ts)
}

fn parse_module<'a> (src: &'a [u8], ts: &mut TokenIter) -> Result<Module<Name>, Report> {
    expect!(ts, TK::KwObject, "A module must start with the keyword `object`.");
    let id1 = expect!(ts, TK::Identifier, "A module must be given a valid identifier name.");
    let name = mkstring(src, id1.range)?;
    let defs = parse_many_definitions(src, ts)?;
    let mexpr = todo!();
    expect!(ts, TK::KwEnd, "A module must end with the keyword `end`.");
    let id2 = expect!(ts, TK::Identifier, "A module must end with its name.");

    if select!(src, id2.range) != select!(src, id1.range) {
        return error!(id2.range, "A module must end with its name.");
    }
    
    Ok(Module { name: name, defs: defs, expr: mexpr })
}

fn parse_many_definitions<'a> (src: &'a [u8], ts: &mut TokenIter) -> Result<VecDeque<Definition<Name>>, Report> {
    match ts.peek().kind {
        TK::KwDef | TK::KwAbstract | TK::KwCase => {
            let def = parse_definition(src, ts)?;
            let mut rest = parse_many_definitions(src, ts)?;
            rest.push_back(def);
            Ok(rest)
        },
        _ => Ok(VecDeque::new())
    }
}

// Parses an abstract class, case class, or function definition
fn parse_definition<'a> (src: &'a [u8], ts: &mut TokenIter) -> Result<Definition<Name>, Report> {
    let kw = ts.pop();
    match kw.kind {
        TK::KwDef => {
            let id = expect!(ts, TK::Identifier, "A function must have a valid name identifier.");
            let args = parse_arglist(src, ts)?;
            expect!(ts, TK::Colon, "Expected a colon after the function argument list.");
            let rt = parse_type(src, ts)?;
            expect!(ts, TK::ColonEqual, "Expected `:=` after the function return type.");
            // parse the function body...
            todo!()
        },
        TK::KwAbstract => {
            expect!(ts, TK::KwClass, "Expected the keyword `class` after `abstract`.");
            let id = expect!(ts, TK::Identifier, "A function must have a valid name identifier.");
            todo!()
        },
        TK::KwCase => {
            expect!(ts, TK::KwClass, "Expected the keyword `class` after `case` in a definition.");
            todo!()
        },
        _ => error!(kw.range, "A definition must start with either `def`, `abstract`, or `case`."),
    }
}

// parses `(x: String, y: Int(32), z: Unit)` or `()`
fn parse_arglist<'a> (src: &'a [u8], ts: &mut TokenIter) -> Result<ArgList<Name>, Report> {
    expect!(ts, TK::OpenParen, "Expected an opening parenthesis to start the argument list.");
    match ts.peek().kind {
        TK::CloseParen => Ok(VecDeque::new()),
        _ => parse_many_arguments(src, ts)
    }
}

// parses a *non-empty* argument list `x: String, y: Int(32), z: Unit \)` <- with the closing parenthesis
fn parse_many_arguments<'a> (src: &'a [u8], ts: &mut TokenIter) -> Result<ArgList<Name>, Report> {
    let arg = parse_argument(src, ts)?;
    let delim = ts.pop();
    match delim.kind {
        TK::Comma => {
            // let arg = parse_argument(src, ts)?;
            let mut lst = parse_many_arguments(src, ts)?;
            lst.push_back(arg);
            Ok(lst)
        }
        TK::CloseParen => {
            let mut lst = VecDeque::new();
            lst.push_back(arg);
            Ok(lst)
        },
        _ => error!(delim.range, "Expected either a comma separator of a closing parenthesis")
    }
}

// parses `x: String` or `y: Int(32)`
fn parse_argument<'a> (src: &'a [u8], ts: &mut TokenIter) -> Result<(Name, Type<Name>), Report> {
    let id = expect!(ts, TK::Identifier, "An argument must have a valid name identifier.");
    let name = mkstring(src, id.range)?;
    expect!(ts, TK::Colon, "Expected a colon after the argument name.");
    let typ = parse_type(src, ts)?;
    Ok((name, typ))
}

// parses `String` or `Unit` or `Int(32)`
fn parse_type<'a> (src: &'a [u8], ts: &mut TokenIter) -> Result<Type<Name>, Report> {
    let typ = ts.pop();
    match typ.kind {
        TK::TypInt => {
            expect!(ts, TK::OpenParen, "The `Int` type must be applied to an integer size value.");
            let size = expect!(ts, TK::LitInt, "Expected an integer literal for the `Int` type.");
            match str::from_utf8(&select!(src, size.range)) {
                Ok("32") => {
                    expect!(ts, TK::CloseParen, "Expected a closing parenthesis.");
                    Ok(Type::IntType)
                },
                _ => error!(size.range, "Only size `32` is supported for integer literals.")
            }
        },
        TK::TypBoolean => Ok(Type::BoolType),
        TK::TypString => Ok(Type::StringType),
        TK::TypUnit => Ok(Type::UnitType),
        TK::Identifier => mkstring(src, typ.range).map(|s| Type::ClassType(s)),
        _ => error!(typ.range, "Expected either a primitive type (`Int`, `Boolean`, `String`, or `Unit`), or an identifier.")
    }
}
