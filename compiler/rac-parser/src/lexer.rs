use std::{iter, str::CharIndices};

use crate::{
    // chars::{is_id_continue, is_id_start},
    token::{Token, TokenKind},
};

pub struct Lexer<'a> {
    chars: CharIndices<'a>,
}

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

    // Peeks n elements ahead
    pub fn peek_n(&mut self, n: usize) -> Vec<char> {
        let mut res: Vec<char> = Vec::with_capacity(n);
        for i in 0..n {
            // match self
        }

        panic!()
    }

    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.char_indices(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let Some((i, first)) = self.bump() else {
            // todo(Harry): make EOF return an accurate range of the end of the file
            return Token { kind: TokenKind::Eof, range: 0..1 };
        };

        let (kind, len) = match first {
            c if is_id_start(c) => {
                let consumed = self.bump_while(is_id_continue);
                (TokenKind::Identifier, consumed)
            }

            '&' if self.peek_first() == '&' => {
                self.bump();
                (TokenKind::AndAnd, 2)
            }
            '!' => (TokenKind::Bang, 1),
            ']' => (TokenKind::CloseBracket, 1),
            ')' => (TokenKind::CloseParen, 1),
            ':' => {
                if self.peek_first() == '=' {
                    self.bump();
                    (TokenKind::ColonEqual, 2)
                } else {
                    (TokenKind::Colon, 1)
                }
            }
            ',' => (TokenKind::Comma, 1),
            '.' => (TokenKind::Dot, 1),
            '=' => {
                if self.peek_first() == '=' {
                    self.bump();
                    (TokenKind::EqualEqual, 2)
                } else {
                    (TokenKind::Equal, 1)
                }
            }
            '<' => {
                if self.peek_first() == '=' {
                    self.bump();
                    (TokenKind::LeftArrow, 2)
                } else {
                    (TokenKind::LessThan, 1)
                }
            }
            '-' => (TokenKind::Minus, 1),
            '[' => (TokenKind::OpenBracket, 1),
            '(' => (TokenKind::OpenParen, 1),
            '%' => (TokenKind::Percent, 1),
            '|' if self.peek_first() == '|' => (TokenKind::PipePipe, 2),
            '+' => {
                if self.peek_first() == '+' {
                    self.bump();
                    (TokenKind::PlusPlus, 2)
                } else {
                    (TokenKind::Plus, 1)
                }
            }
            ';' => (TokenKind::Semicolon, 1),
            '/' => (TokenKind::Slash, 1),
            '*' => (TokenKind::Star, 1),

            _ => (TokenKind::Unknown, 1),
        };

        Token { kind, range: i..(i + len) }
    }
}

pub fn is_id_start(c: char) -> bool {
    c.is_ascii_alphabetic()
}

pub fn is_id_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut lexer = Lexer::new(input);

    iter::from_fn(move || {
        let tok = lexer.next_token();
        if tok.kind != TokenKind::Eof {
            Some(tok)
        } else {
            None
        }
    })
}
