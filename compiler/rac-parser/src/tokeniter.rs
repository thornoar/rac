
use crate::{
    // chars::{is_id_continue, is_id_start},
    token::{Token},
};

#[derive(Clone, Copy, Debug)]
pub struct TokenIter<'a> {
    src: &'a str,
    start: usize
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        panic!()
    }
}

fn lex_token(src: &str, start: usize) -> Option<Token> {
    panic!()
}
