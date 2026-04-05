mod cli;
mod file;
mod repl;

fn main() -> std::io::Result<()> {
    crate::cli::Cli::run()
}
