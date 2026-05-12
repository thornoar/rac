use crate::lexer::Lexer;

impl<'a> Lexer<'a> {
    pub const EOF_CHAR: char = '\0';

    pub fn bump(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    pub fn bump_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> usize {
        let mut consumed = 0;
        while predicate(self.peek_first()) {
            self.bump();
            consumed += 1;
        }

        consumed
    }

    pub fn peek_first(&mut self) -> char {
        self.chars
            .clone()
            .next()
            .map(|tup| tup.1)
            .unwrap_or(Self::EOF_CHAR)
    }
}

pub fn is_id_start(c: char) -> bool {
    c.is_ascii_alphabetic()
}

pub fn is_id_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
