use std::ops::RangeInclusive;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Eof,
    Identifier,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub range: (u32, u32), // todo(Harry): make a dedicated span type
}

impl Token {
    pub fn new(kind: TokenKind, range: RangeInclusive<u32>) -> Self {
        Self {
            kind,
            range: (*range.start(), *range.end()),
        }
    }
}
