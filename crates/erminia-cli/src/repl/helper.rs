use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper};

pub struct ReplHelper {
    pub file_completer: FilenameCompleter,
}

impl Helper for ReplHelper {}

const COMMANDS: &[&str] = &[
    "help", "exit", "clear", "ls", "from ", ":parse", ":tokens", ":cat",
];

impl Completer for ReplHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let input = &line[..pos];

        // File completion after "from "
        if let Some(path_part) = input.strip_prefix("from ") {
            let (repl_offset, candidates) =
                self.file_completer
                    .complete(path_part, path_part.len(), ctx)?;
            return Ok(("from ".len() + repl_offset, candidates));
        }

        // Command name completion
        let candidates: Vec<Pair> = COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(input))
            .map(|cmd| Pair {
                display: cmd.to_string(),
                replacement: cmd.to_string(),
            })
            .collect();

        Ok((0, candidates))
    }
}

impl Hinter for ReplHelper {
    type Hint = String;
    fn hint(&self, _line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<String> {
        None
    }
}

impl Highlighter for ReplHelper {}
impl Validator for ReplHelper {}
