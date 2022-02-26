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
            let mut g = Git::new();
            // println!("{:?}", g.get_current_branch().unwrap());
            println!("{:?}", g.get_recent_branches(0).unwrap());
        },
    }
}
