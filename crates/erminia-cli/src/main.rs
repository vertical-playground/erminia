use erminia::{diagnostics::DiagnosticAccumulator, syntax::Parser};
use std::io::{self, Write};

mod file;

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut stdout = io::stdout();
    let mut diag: DiagnosticAccumulator = DiagnosticAccumulator::new();

    let mut input = String::new();

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;

        input.clear();
        stdin.read_line(&mut input)?;

        if input == "exit\n" {
            std::process::exit(0);
        }

        if input == "clear\n" {
            println!("\033[H\033[2J");
            continue;
        }

        if input.starts_with("from ") {
            input = file::io::from_file(
                input.to_string().drain(0..4).as_str().to_string(),
                &mut diag,
            );
        }

        let mut parser = Parser::new(&input);

        let program = parser.parse();

        if program.is_err() {
            for diag in parser
                .get_diagnostics()
                .get(erminia::config::CompilerPass::Parser)
            {
                eprintln!("{}", diag);
            }
            continue;
        }

        println!("{:?}", program);

        // parse to AST Tree
        // check semantics
        // generate json
    }
}
