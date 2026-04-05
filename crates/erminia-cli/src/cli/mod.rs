use clap::{Parser, Subcommand, ValueEnum};
use erminia::{config::CompilerPass, diagnostics::DiagnosticAccumulator, lexer::lex::Lexer};

use crate::repl::colors::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Print parsed AST (default)
    Parse,
    /// Print token listing
    Tokens,
    /// Print AST in debug format
    AstDebug,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Runs a compile pass based on some mode
    Run {
        /// Input file
        input: String,
        /// Output file
        output: Option<String>,
        /// Output mode
        #[arg(short, long, value_enum, default_value = "parse")]
        mode: Mode,
    },
}

#[derive(Parser)]
#[command(name = "erminia")]
#[command(version, about = "The Erminia language toolkit", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

impl Cli {
    pub fn run() -> std::io::Result<()> {
        let cli = Cli::parse();

        if let Some(command) = cli.command {
            match command {
                Commands::Run {
                    input,
                    output,
                    mode,
                } => {
                    let mut diag = DiagnosticAccumulator::new();
                    if let Ok(source) = crate::file::io::from_file(input, &mut diag) {
                        run_source(&source, mode, output)
                    } else {
                        eprintln!("{}", diag);
                        Ok(())
                    }
                }
            }
        } else {
            crate::repl::engine::ErminiaREPL::new().run()
        }
    }
}

fn run_source(source: &str, mode: Mode, output: Option<String>) -> std::io::Result<()> {
    match mode {
        Mode::Parse => {
            let mut parser = erminia::syntax::Parser::new(source);
            let program = parser.parse();
            if program.is_ok() {
                if let Some(out) = output {
                    let _ = std::fs::write(out, format!("{}{:?}{}", GREEN, program, RESET));
                } else {
                    println!("{}{:?}{}", GREEN, program, RESET);
                }
            } else {
                for diag in parser.get_diagnostics().get(CompilerPass::Parser) {
                    eprintln!("{}{}{}", RED, diag, RESET);
                }
            }
        }
        Mode::Tokens => {
            let mut lexer = Lexer::new(source);
            let tokens = lexer.lex_with_separate_pass();
            if let Some(out) = output {
                let _ = std::fs::write(out, format!("{}  {:?}{}", CYAN, tokens, RESET));
            } else {
                println!("{}  {:?}{}", CYAN, tokens, RESET);
            }
        }
        Mode::AstDebug => {
            let mut parser = erminia::syntax::Parser::new(source);
            let program = parser.parse();
            if let Some(out) = output {
                let _ = std::fs::write(out, format!("{}{:#?}{}", CYAN, program, RESET));
            } else {
                println!("{}{:#?}{}", CYAN, program, RESET);
            }
        }
    }

    Ok(())
}
