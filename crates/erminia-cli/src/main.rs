mod file;
mod repl;

fn main() -> std::io::Result<()> {
    repl::engine::ErminiaREPL::new().run()
}
