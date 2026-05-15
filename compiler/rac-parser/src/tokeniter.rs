use crate::token::{Token, TokenKind};

#[derive(Clone, Copy, Debug)]
pub struct TokenIter<'a> {
    src: &'a [u8],
    limit: usize,
    position: usize
}

impl<'a> TokenIter<'a> {
    pub fn new(src: &'a [u8], limit: usize) -> Self {
        Self { src, limit, position: 0 }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = lex_token(self.src, self.limit, self.position);
        self.position = tok.range.end;

        if tok.kind == TokenKind::Eof {
            None
        } else {
            Some(tok)
        }
    }
}

// Produce the next token from the `start` position.
fn lex_token(src: &[u8], limit: usize, start: usize) -> Token {
    use TokenKind::*;

    // Check if we have any characters left
    if start >= limit {
        return Token::new(Eof, start..start);
    }

    let span = |l| {
        return start .. (start + l);
    };
    let has_next: bool = start+1 < limit;

    match src[start] {
        // Skip whitespace
        c if c.is_ascii_whitespace() => lex_token(src, limit, start+1),
        c if is_id_start(c) => {
            let mut end: usize = start+1;
            while end < limit && is_id_continue(src[end]) {
                end += 1;
            }
            let tk: TokenKind = match str::from_utf8(&src[start..end]) {
                // Keywords
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
                // Primitive types
                Ok("String") => TypString,
                Ok("Int") => if end + 4 <= limit && let Ok("(32)") = str::from_utf8(&src[end..(end+4)]) {
                    end += 4;
                    TypInt32
                } else { Unknown },
                Ok("Boolean") => TypBoolean,
                Ok("Unit") => TypUnit,
                // Literals
                Ok("true") => LitTrue,
                Ok("false") => LitFalse,
                // Otherwise, it's an identifier
                Ok(_) => Identifier,
                Err(_) => Unknown
            };
            Token::new(tk, start..end)
        }

        b'&' if has_next && src[start+1] == b'&' => Token::new(AndAnd, span(2)),
        b'!' => Token::new(Bang, span(1)), // [
        b']' => Token::new(CloseBracket, span(1)), // (
        b')' => Token::new(CloseParen, span(1)),
        b':' => {
            if has_next && src[start+1] == b'=' {
                Token::new(ColonEqual, span(2))
            } else {
                Token::new(Colon, span(1))
            }
        },
        b',' => Token::new(Comma, span(1)),
        b'.' => Token::new(Dot, span(1)),
        b'=' => {
            if has_next && src[start+1] == b'=' {
                Token::new(EqualEqual, span(2))
            } else {
                Token::new(Equal, span(1))
            }
        },
        b'<' => {
            if has_next && src[start+1] == b'=' {
                Token::new(LeftArrow, span(2))
            } else {
                Token::new(LessThan, span(1))
            }
        },
        b'-' => Token::new(Minus, span(1)),
        b'[' => Token::new(OpenBracket, span(1)),
        b'(' => {
            let mut end = start + 1;
            while end < limit && src[end].is_ascii_whitespace() {
                end += 1;
            }
            if end < limit && src[end] == b')' {
                Token::new(LitUnit, start .. end)
            } else {
                Token::new(OpenParen, span(1))
            }
        },

        b'%' => Token::new(Percent, span(1)),
        b'|' if has_next && src[start+1] == b'|' => Token::new(PipePipe, span(2)),
        b'+' => {
            if has_next && src[start+1] == b'+' {
                Token::new(PlusPlus, span(2))
            } else {
                Token::new(Plus, span(1))
            }
        }
        b';' => Token::new(Semicolon, span(1)),
        b'/' => Token::new(Slash, span(1)),
        b'*' => Token::new(Star, span(1)),

        _ => Token::new(Unknown, span(1)),
    }
}

pub fn is_id_start(c: u8) -> bool {
    c.is_ascii_alphabetic()
}

pub fn is_id_continue(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_'
}
