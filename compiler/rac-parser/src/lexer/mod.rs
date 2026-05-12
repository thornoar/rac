use std::{iter, str::CharIndices};

use crate::lexer::{
    chars::{is_id_continue, is_id_start},
    token::{Token, TokenKind},
};

mod chars;
pub mod token;

pub struct Lexer<'a> {
    chars: CharIndices<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.char_indices(),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let Some((i, first)) = self.bump() else {
            // todo(Harry): make EOF return an accurate range of the end of the file
            return Token::new(TokenKind::Eof, 0..1);
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

        Token::new(kind, i..(i + len))
    }
}

pub fn lex(input: &str) -> impl Iterator<Item = Token> + '_ {
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
