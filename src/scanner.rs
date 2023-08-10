use crate::error_handler::ProxError;
use crate::utils::sub_string::StringUtils;
use crate::{
    token::Token,
    token_kind::{Literals, TokenKind},
};

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    fn add_token(&mut self, kind: TokenKind) {
        self.set_token(kind, None);
    }

    fn advance(&self) -> char {
        self.source.chars().next().unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current as usize >= self.source.len()
    }

    pub fn new(source: String) -> Scanner {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn set_token(&mut self, kind: TokenKind, literal: Option<Literals>) {
        let text: String = self
            .source
            .sub_string(self.start as usize, self.current as usize)
            .to_string();
        self.tokens.push(Token::new(kind, text, literal, self.line))
    }

    fn scan_token(&mut self) -> Result<(), ProxError> {
        let ch: char = self.advance();
        match ch {
            '(' => Ok(self.add_token(TokenKind::LeftParen)),
            ')' => Ok(self.add_token(TokenKind::RightParen)),
            '{' => Ok(self.add_token(TokenKind::LeftBrace)),
            '}' => Ok(self.add_token(TokenKind::RightBrace)),
            ',' => Ok(self.add_token(TokenKind::Comma)),
            '.' => Ok(self.add_token(TokenKind::Dot)),
            '-' => Ok(self.add_token(TokenKind::Minus)),
            '+' => Ok(self.add_token(TokenKind::Plus)),
            ';' => Ok(self.add_token(TokenKind::SemiColon)),
            '*' => Ok(self.add_token(TokenKind::Star)),
            _ => Err(ProxError::new(self.line, "Unexpected character")),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenKind::Eof, String::new(), None, self.line));
        self.tokens.clone()
    }
}
