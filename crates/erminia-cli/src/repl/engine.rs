use super::colors::*;
use super::command::{Command, OutputMode};
use super::helper::ReplHelper;
use crate::file;
use erminia::{
    config::CompilerPass, diagnostics::DiagnosticAccumulator, lexer::lex::Lexer, syntax::Parser,
};
use rustyline::completion::FilenameCompleter;
use rustyline::history::DefaultHistory;
use rustyline::{CompletionType, Editor, config::Configurer, error::ReadlineError};
use std::io;
use std::path::PathBuf;

// ==================================================================================== //
// Utilities                                                                            //
// ==================================================================================== //

fn history_path() -> Option<PathBuf> {
    std::env::var("HOME")
        .ok()
        .map(|home| PathBuf::from(home).join(".erminia_history"))
}

/// Returns true when all bracket/paren/brace delimiters are balanced.
fn is_balanced(input: &str) -> bool {
    let mut depth: i32 = 0;
    for c in input.chars() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' => depth -= 1,
            _ => {}
        }
    }
    depth == 0
}

// ==================================================================================== //
// REPL                                                                                 //
// ==================================================================================== //

pub struct ErminiaREPL {
    diagnostics: DiagnosticAccumulator,
    mode: OutputMode,
}

impl ErminiaREPL {
    pub fn new() -> Self {
        ErminiaREPL {
            diagnostics: DiagnosticAccumulator::new(),
            mode: OutputMode::Parse,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let helper = ReplHelper {
            file_completer: FilenameCompleter::new(),
        };
        let mut rl = Editor::<ReplHelper, DefaultHistory>::new().map_err(io::Error::other)?;
        rl.set_completion_type(CompletionType::List);
        rl.set_helper(Some(helper));

        if let Some(path) = history_path() {
            let _ = rl.load_history(&path);
        }

        println!(
            "{}{}Erminia REPL{} — type {}help{} for available commands",
            BOLD, CYAN, RESET, BOLD, RESET
        );

        loop {
            match rl.readline(">> ") {
                Ok(line) => {
                    let _ = rl.add_history_entry(&line);

                    // Collect continuation lines until brackets are balanced.
                    let mut source = line;
                    while !is_balanced(&source) {
                        match rl.readline(".. ") {
                            Ok(cont) => {
                                let _ = rl.add_history_entry(&cont);
                                source.push('\n');
                                source.push_str(&cont);
                            }
                            Err(ReadlineError::Interrupted | ReadlineError::Eof) => break,
                            Err(e) => return Err(io::Error::other(e)),
                        }
                    }

                    match Command::from_input(&source) {
                        Command::Exit => {
                            if let Some(path) = history_path() {
                                let _ = rl.save_history(&path);
                            }
                            return Ok(());
                        }
                        Command::Clear => rl.clear_screen().map_err(io::Error::other)?,
                        Command::Help => self.print_help(),
                        Command::SetMode(mode) => {
                            self.mode = mode;
                            println!("{}mode: {:?}{}", CYAN, mode, RESET);
                        }
                        Command::From(filename) => {
                            match file::io::from_file(filename, &mut self.diagnostics) {
                                Ok(contents) => self.eval(&contents),
                                Err(_) => eprintln!("{}{}{}", RED, self.diagnostics, RESET),
                            }
                        }
                        Command::Ls(path) => {
                            let dir = path.as_deref().unwrap_or(".");
                            match std::fs::read_dir(dir) {
                                Ok(entries) => {
                                    let mut names: Vec<String> = entries
                                        .filter_map(|e| e.ok())
                                        .map(|e| {
                                            let name = e.file_name().to_string_lossy().to_string();
                                            if e.path().is_dir() {
                                                format!("{}/", name)
                                            } else {
                                                name
                                            }
                                        })
                                        .collect();
                                    names.sort();
                                    for name in &names {
                                        println!("  {}", name);
                                    }
                                }
                                Err(e) => eprintln!("{}ls: {}{}", RED, e, RESET),
                            }
                        }
                        Command::Input(ref s) if !s.is_empty() => self.eval(&source),
                        Command::Input(_) => {}
                    }
                }
                Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                    if let Some(path) = history_path() {
                        let _ = rl.save_history(&path);
                    }
                    return Ok(());
                }
                Err(e) => return Err(io::Error::other(e)),
            }
        }
    }

    fn print_help(&self) {
        println!("{}{}Commands:{}", BOLD, CYAN, RESET);
        println!("  {BOLD}help{RESET}           — show this message");
        println!("  {BOLD}exit{RESET}           — quit the REPL");
        println!("  {BOLD}clear{RESET}          — clear the screen");
        println!("  {BOLD}ls [path]{RESET}      — list directory contents");
        println!("  {BOLD}from <file>{RESET}    — load and evaluate a file");
        println!(
            "  {BOLD}:cat{RESET}           — switch to cat mode (print file contents without evaluating)"
        );
        println!(
            "  {BOLD}:parse{RESET}         — switch to parse/AST output mode {YELLOW}(default){RESET}"
        );
        println!("  {BOLD}:tokens{RESET}        — switch to token listing mode");
        println!("  {BOLD}Ctrl-C / Ctrl-D{RESET} — quit");
        println!("  {BOLD}Tab{RESET}            — show available commands / complete paths");
        println!("\n  current mode: {YELLOW}{:?}{RESET}", self.mode);
    }

    fn eval(&self, source: &str) {
        match self.mode {
            OutputMode::Parse => self.eval_parse(source),
            OutputMode::Tokens => self.eval_tokens(source),
            OutputMode::Cat => self.eval_cat(source),
        }
    }

    fn eval_parse(&self, source: &str) {
        let mut parser = Parser::new(source);
        let program = parser.parse();

        if program.is_ok() {
            println!("{}{:?}{}", GREEN, program, RESET);
        } else {
            for diag in parser.get_diagnostics().get(CompilerPass::Parser) {
                eprintln!("{}{}{}", RED, diag, RESET);
            }
        }
    }

    fn eval_tokens(&self, source: &str) {
        let mut lexer = Lexer::new(source);
        let tokens = lexer.lex_with_separate_pass();
        for token in &tokens {
            println!("{}  {}{}", CYAN, token, RESET);
        }
    }

    fn eval_cat(&self, source: &str) {
        println!("{}", source);
    }
}
