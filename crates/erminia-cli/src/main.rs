use erminia::lexer::lex_s::Lexer;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let mut lexer = Lexer::new(&input);
        let tokens = lexer.lex();

        println!("{:?}", tokens);

        // parse to AST Tree
        // check semantics
        // generate json

        input.clear();
    }
}
