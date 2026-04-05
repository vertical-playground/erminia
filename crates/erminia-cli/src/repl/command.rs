#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputMode {
    Parse,
    Tokens,
    Cat,
}

pub enum Command {
    Exit,
    Clear,
    Help,
    SetMode(OutputMode),
    From(String),
    Ls(Option<String>),
    Input(String),
}

impl Command {
    pub fn from_input(input: &str) -> Self {
        let trimmed = input.trim();
        match trimmed {
            "exit" | ":exit" => Command::Exit,
            "clear" | ":clear" => Command::Clear,
            "help" | ":help" => Command::Help,
            ":parse" => Command::SetMode(OutputMode::Parse),
            ":tokens" => Command::SetMode(OutputMode::Tokens),
            ":cat" => Command::SetMode(OutputMode::Cat),
            _ if trimmed.starts_with("from ") => {
                Command::From(trimmed["from ".len()..].to_string())
            }
            "ls" => Command::Ls(None),
            _ if trimmed.starts_with("ls ") => {
                Command::Ls(Some(trimmed["ls ".len()..].trim().to_string()))
            }
            _ => Command::Input(trimmed.to_string()),
        }
    }
}
