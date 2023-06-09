use std::io;
use std::io::Write;

mod lexer;

fn main() {
    print!("> ");

    let mut s = String::new();
    input(&mut s);

    let l = lexer::Lexer::new(s.clone());
    let tokens = Vec::new();
    let token = l.next_token();

    while matches!(token.kind, lexer::TokenKind::End) {
        tokens.push(token);
        token = l.next_token();
    }

    println!("{:?}", tokens);
}

fn input(s: &mut String) {
    std::io::stdout().flush().expect("couldn't flush stdout");

    let stdin = io::stdin();
    stdin.read_line(s).expect("couldn't read user input");
}

