use std::ops::Range;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    // Identifiers
    Identifier,

    // Delimiters
    AndAnd,
    Bang,
    CloseBracket,
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
    OpenBracket,
    OpenParen,
    Percent,
    PipePipe,
    Plus,
    PlusPlus,
    Semicolon,
    Slash,
    Star,

    // Primitive types
    TypString,
    TypInt,
    TypBoolean,
    TypUnit,

    // Built-in literal values for Boolean and Unit types
    LitTrue,
    LitFalse,
    LitUnit,

    // Keywords
    KwAbstract,
    KwCase,
    KwClass,
    KwDef,
    KwExtends,
    KwIf,
    KwThen,
    KwElse,
    KwMatch,
    KwObject,
    KwVal,
    KwError,
    KwEnd,

    // Misc
    Unknown,
    Eof,
    Underscore,
}

pub type Span = Range<usize>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub range: Span,
}

impl Token {
    pub fn new(kind: TokenKind, range: Range<usize>) -> Self {
        Self {
            kind,
            range
        }
    }
}
