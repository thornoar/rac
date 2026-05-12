use crate::lexer::Lexer;

impl<'a> Lexer<'a> {
    pub const EOF_CHAR: char = '\0';

    pub fn bump(&mut self) -> Option<(usize, char)> {
        self.chars.next()
    }

    pub fn bump_while<F: Fn(char) -> bool>(&mut self, predicate: F) -> usize {
        let mut consumed = 0;
        while predicate(self.first().1) {
            self.bump();
            consumed += 1;
        }

        consumed
    }

    pub fn first(&mut self) -> (usize, char) {
        self.chars
            .clone()
            .next()
            .unwrap_or((usize::MAX, Self::EOF_CHAR))
    }
}

pub fn is_id_start(c: char) -> bool {
    c.is_ascii_alphabetic()
}

pub fn is_id_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
