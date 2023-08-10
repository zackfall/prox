use crate::token_kind::{Literals, TokenKind};

#[derive(Clone)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    literal: Option<Literals>,
    line: u64,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, literal: Option<Literals>, line: u64) -> Token {
        Token {
            kind,
            lexeme,
            literal,
            line,
        }
    }
}

impl ToString for Token {
    fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.kind, self.lexeme, self.literal)
    }
}
