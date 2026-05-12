use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Identifier,

    AndAnd,
    Bang,
    CloseParen,
    Colon,
    ColonEqual,
    Comma,
    Dot,
    Equal,
    EqualEqual,
    LeftArrow,
    LessThan,
    Minus,
    OpenParen,
    Percent,
    PipePipe,
    Plus,
    PlusPlus,
    Semicolon,
    Slash,
    Star,

    Eof,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub range: (usize, usize), // todo(Harry): make a dedicated span type
}

impl Token {
    pub fn new(kind: TokenKind, range: Range<usize>) -> Self {
        Self {
            kind,
            range: (range.start, range.end - 1),
        }
    }
}
