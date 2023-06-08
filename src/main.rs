use std::io;
use std::io::Write;

enum TokenKind {
    Number,
    Identifier,

    Plus,
    Minus,
    Times,
    Divide,

    End,
    Error
}

struct Token {
    content: String,
    kind: TokenKind
}

struct Lexer {
   content: Vec<char>,
   cursor: usize
}

impl Lexer {
    fn new(content: String) -> Self {
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

    fn next_token(&mut self) -> Token {
        if self.char().is_none() {
            Token { content: "".into(), kind: TokenKind::End }
        }

        
    }
}

fn main() {
    print!("> ");

    let mut s = String::new();
    input(&mut s);

    let l = Lexer::new(s.clone());

    println!("{}", s);
}

fn input(s: &mut String) {
    std::io::stdout().flush().expect("couldn't flush stdout");

    let stdin = io::stdin();
    stdin.read_line(s).expect("couldn't read user input");
}

