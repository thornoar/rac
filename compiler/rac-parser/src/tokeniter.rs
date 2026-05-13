use crate::{
    // chars::{is_id_continue, is_id_start},
    token::{Token, TokenKind},
};

#[derive(Clone, Copy, Debug)]
pub struct TokenIter<'a> {
    src: &'a str,
    start: usize
}

impl<'a> TokenIter<'a> {
    pub fn new(src: &'a str) -> Self {
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

fn lex_token(src: &str, start: usize) -> Token {
    panic!()
}
