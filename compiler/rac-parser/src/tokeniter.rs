use crate::token::{Span, Token, TokenKind};

#[derive(Clone, Copy, Debug)]
pub struct TokenIter<'a> {
    src: &'a [u8],
    start: usize
}

impl<'a> TokenIter<'a> {
    pub fn new(src: &'a [u8]) -> Self {
        Self { src, start: 0 }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = lex_token(self.src, self.start);
        self.start += tok.range.len();

        if tok.kind == TokenKind::Eof {
            None
        } else {
            Some(tok)
        }
    }
}

fn lex_token(src: &[u8], start: usize) -> Token {
    use TokenKind::*;
    let span = |l| {
        return start .. (start + l);
    };
    match src[start] {
        c if is_id_start(c) => {
            let mut end: usize = start+1;
            while is_id_continue(src[end]) {
                end += 1;
            }
            let tk: TokenKind = match str::from_utf8(&src[start..end]) {
                Ok("abstract") => KwAbstract,
                Ok("case") => KwCase,
                Ok("class") => KwClass,
                Ok("def") => KwDef,
                Ok("else") => KwElse,
                Ok("extends") => KwExtends,
                Ok("if") => KwIf,
                Ok("then") => KwThen,
                Ok("match") => KwMatch,
                Ok("object") => KwObject,
                Ok("val") => KwVal,
                Ok("error") => KwError,
                Ok("end") => KwEnd,
                Ok("_") => Underscore,
                Ok(_) => Identifier,
            };
            Token { kind: tk, range: start..end }
        }

        b'&' if src[start+1] == b'&' => Token { kind: AndAnd, range: span(2) },
        b'!' => Token { kind: Bang, range: span(1) }, // [
        b']' => Token { kind: CloseBracket, range: span(1) }, // (
        b')' => Token { kind: CloseParen, range: span(1) },
        b':' => {
            if src[start+1] == b'=' {
                Token { ColonEqual, span(2) }
            } else {
                Token { Colon, span(1) }
            }
        },
        b',' => Token { kind: Comma, range: span(1) },
        b'.' => Token { kind: Dot, range: span(1) },
        b'=' => {
            if src[start+1] == '=' {
                Token { kind: EqualEqual, range: span(2) }
            } else {
                Token { kind: Equal, range: span(1) }
            }
        },
        b'<' => {
            if src[start+1] == '=' {
                Token { kind: LeftArrow, range: span(2) }
            } else {
                Token { kind: LessThan, range: span(1) }
            }
        },
        b'-' => Token { kind: Minus, range: span(1) },
        b'[' => Token { kind: OpenBracket, range: span(1) },
        b'(' => Token { kind: OpenParen, range: span(1) },
        b'%' => Token { kind: Percent, range: span(1) },
        b'|' if src[start+1] == b'|' => Token { kind: PipePipe, range: span(2) },
        b'+' => {
            if src[start+1] == '+' {
                Token { kind: PlusPlus, range: span(2) }
            } else {
                Token { kind: Plus, range: span(1) }
            }
        }
        b';' => Token { kind: Semicolon, range: span(1) },
        b'/' => Token { kind: Slash, range: span(1) },
        b'*' => Token { kind: Star, range: span(1) },

        _ => Token { kind: Unknown, range: span(1) },
    }
}

pub fn is_id_start(c: u8) -> bool {
    c.is_ascii_alphabetic()
}

// pub fn bytes_to_str(bytes: &[u8]) -> &str {
//
// }

pub fn is_id_continue(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}
