use std::{ops::Deref, io::stdin, process::Command};

use git_ctx::{Cli, Git};
use clap::Parser;

fn main() {
    let args = Cli::parse();

    match args.command {
        git_ctx::Commands::ListBranches { limit } => {
            let mut g = Git::new();
            let branches = g.get_recent_branches(limit).unwrap();
            let current_branch = g.get_current_branch().unwrap();
            println!("{}", current_branch);
            for branch in branches {
                if branch == current_branch {
                    println!("[*] {}", branch);
                } else {
                    println!("  {}", branch);
                }
            }
        },
        git_ctx::Commands::SwitchBranch { limit } => {
            let mut g = Git::new();
            let branches = g.get_recent_branches(limit).unwrap();
            let current_branch = g.get_current_branch().unwrap();

            for (i, b) in branches.iter().enumerate() {
                let is_current_branch = current_branch == b.deref();

                println!(
                    "[{}] {}{}{}",
                    i,
                    if is_current_branch { "-->" } else { "" },
                    b,
                    if is_current_branch { "<--" } else { "" },
                )
            }
            println!("---------------------");
            println!("Enter the branch number you want to switch to: ");
            let mut user_input = String::new();
            stdin().read_line(&mut user_input).expect("invalid input");
            let num: usize = user_input.trim().parse().expect("invalid input");
            let branch = &branches[num];

            let output = Command::new("git")
                .args(["checkout", branch])
                .output()
                .expect("failed to execute the git command");
            println!("{:?}", output);
        },
    }
}
