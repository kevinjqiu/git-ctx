use git_ctx::{Cli, Git};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        git_ctx::Commands::ListBranches {  } => {
            let _g = Git::new();
            println!("list")
        },
        git_ctx::Commands::SwitchBranch {  } => {
            let _g = Git::new();
            println!("switch")
        },
    }
}
