use git_ctx::{Cli, Git};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        git_ctx::Commands::ListBranches { limit } => {
            let mut g = Git::new();
            println!("{:?}", g.get_recent_branches(limit).unwrap());
        },
        git_ctx::Commands::SwitchBranch {  } => {
            let _g = Git::new();
            println!("switch")
        },
    }
}
