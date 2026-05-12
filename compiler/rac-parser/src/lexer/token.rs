use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Eof,
    Identifier,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub range: (usize, usize), // todo(Harry): make a dedicated span type
}

impl Token {
    pub fn new(kind: TokenKind, range: RangeInclusive<usize>) -> Self {
        Self {
            kind,
            range: (*range.start(), *range.end()),
        }
    }
}
