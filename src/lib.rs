pub mod tui;

use clap::{Parser, Subcommand};
use log::info;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::Path;
use std::{env, path::PathBuf};

#[derive(Debug)]
pub enum Error {
    MissingHeadLog,
    MalformedCheckoutLog,
    NoCurrentBranch,
    IOError(std::io::Error),
}

type Result<T> = core::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IOError { 0: err }
    }
}

type BranchHistory = HashMap<String, u32>;

#[derive(Parser)]
#[clap(name = "git-ctx")]
#[clap(about="git context switching", long_about=None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(alias = "l")]
    ListBranches {
        #[clap(short, default_value = "10")]
        limit: usize,
    },

    #[clap(alias = "s")]
    SwitchBranch {
        #[clap(short, default_value = "10")]
        limit: usize,
    },
    #[clap(alias = "u")]
    ShowTui {
        #[clap(short, default_value = "10")]
        limit: usize,
    }
}

#[derive(Debug)]
pub struct Git {
    git_folder: PathBuf,
}

impl Git {
    fn find_first_path_with_git_folder(folder: Option<PathBuf>) -> Option<PathBuf> {
        if let Some(folder) = folder {
            let git_path = format!("{}/.git", folder.to_str().unwrap());
            let p = Path::new(&git_path).to_owned();
            if p.exists() {
                return Some(p);
            }

            if let Some(parent) = folder.parent() {
                return Git::find_first_path_with_git_folder(Some(parent.to_owned()));
            }
        }
        None
    }

    pub fn new() -> Self {
        let cwd = env::current_dir().unwrap();
        let git_folder =
            Git::find_first_path_with_git_folder(Some(cwd)).expect("unable to find .git folder");
        Git { git_folder }
    }

    pub fn get_current_branch(&mut self) -> Result<String> {
        let f = self.git_folder.join("HEAD");

        let head_file = File::open(f)?;
        let mut buf_reader = BufReader::new(head_file);
        let mut content = String::new();
        buf_reader.read_to_string(&mut content)?;
        match content.trim().split("/").last() {
            Some(s) => Ok(String::from(s)),
            None => Err(Error::NoCurrentBranch),
        }
    }

    fn parse_head_log(&mut self) -> Result<BranchHistory> {
        let mut ret: BranchHistory = BranchHistory::new();
        let f = self.git_folder.join("logs/HEAD");

        if !f.exists() {
            return Err(Error::MissingHeadLog);
        }

        let head_log_file = File::open(f)?;
        let mut buf_reader = BufReader::new(head_log_file);
        let mut seq = 0;

        loop {
            let mut line = String::new();
            let bytes_read = buf_reader.read_line(&mut line)?;
            if bytes_read == 0 {
                break;
            }
            info!("line={}", line);
            match line.trim().split('\t').nth(1) {
                Some(msg) => {
                    if !msg.starts_with("checkout: moving from ") {
                        continue;
                    }

                    let msg = &msg["checkout: moving from ".len()..msg.len()];

                    let parts: Vec<&str> = msg.split(" to ").collect();
                    if parts.len() != 2 {
                        return Err(Error::MalformedCheckoutLog);
                    }

                    ret.insert(String::from(parts[0]), seq);
                    ret.insert(String::from(parts[1]), seq + 1);
                    seq += 1;
                }
                None => continue,
            }
        }

        Ok(ret)
    }

    pub fn get_recent_branches(&mut self, limit: usize) -> Result<Vec<String>> {
        let branch_history = self.parse_head_log()?;

        let mut items_vec: Vec<(&String, &u32)> = branch_history.iter().collect();
        items_vec.sort_by_key(|k| k.1);
        items_vec.reverse();

        let branches = items_vec
            .into_iter()
            .map(|(branch, _seq)| branch.clone())
            .take(limit)
            .collect();

        Ok(branches)
    }
}
