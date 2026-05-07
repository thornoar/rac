#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenKind {
    Eof,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: (u32, u32),   // todo(Harry): make a dedicated span type
}
