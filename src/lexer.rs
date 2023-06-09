#[derive(Debug)]
pub enum TokenKind {
    Number,
    Identifier,

    Plus,
    Minus,
    Times,
    Divide,

    End,
    Error
}

#[derive(Debug, PartialEq, Eq)]
struct Token {
    content: String,
    pub kind: TokenKind
}

pub struct Lexer {
   content: Vec<char>,
   cursor: usize
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self { content: content.chars().collect(), cursor: 0 }
    }

    fn advance(&mut self) {
        self.cursor += 1;
    }

    fn char(&self) -> Option<char> {
        if self.content.get(self.cursor).is_none() {
            None
        }
        else {
            Some(*self.content.get(self.cursor).unwrap())
        }
    }

    pub fn next_token(&mut self) -> Token {
        if self.char().is_none() {
            return Token { content: "".into(), kind: TokenKind::End };
        }
        
        if let Some(ch) = self.char() {
            if ch.is_alphabetic() {
                let pos = self.cursor;

                while !self.char().is_none() && Some(self.char()).unwrap().expect("is None!").is_alphabetic() {
                    self.advance();
                }
                
                return Token { content: self.content[pos..self.cursor].iter().collect(), kind: TokenKind::Identifier };
            }
        }

        Token { content: "Unknown Token".into(), kind: TokenKind::Error }
    }
}

