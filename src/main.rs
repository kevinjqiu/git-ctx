use clap::Parser;
use git_ctx::{Cli};

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match args.command {
        Some(git_ctx::Commands::ShowTui { limit }) => {
            git_ctx::tui::run_tui(limit).unwrap();
            Ok(())
        }
        None => {
            git_ctx::tui::run_tui(20).unwrap();
            Ok(())
        }
    }
}
