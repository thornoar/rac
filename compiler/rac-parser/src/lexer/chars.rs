use crate::lexer::Lexer;

impl<'a> Lexer<'a> {
    pub const EOF_CHAR: char = '\0';

    pub fn bump(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    pub fn first(&mut self) -> (usize, char) {
        self.chars
            .clone()
            .next()
            .unwrap_or((usize::MAX, Self::EOF_CHAR))
    }
}
