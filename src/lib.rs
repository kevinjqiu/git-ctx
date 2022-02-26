use log::{info};
use std::io::{BufReader, Read, BufRead};
use std::{env, path::PathBuf};
use std::path::Path;
use std::fs::File;
use clap::{Parser, Subcommand};

#[derive(Debug)]
pub enum Error {
	MissingHeadLog,
	MalformedCheckoutLog,
	IOError(std::io::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
		Error::IOError{0: err}
    }
}

#[derive(Debug)]
pub struct BranchHistory {
	pub from: String,
	pub to: String,
}

#[derive(Parser)]
#[clap(name="git-ctx")]
#[clap(about="git context switching", long_about=None)]
pub struct Cli {
	#[clap(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	#[clap()]
	ListBranches {

	},

	#[clap()]
	SwitchBranch {

	}
}

#[derive(Debug)]
pub struct Git {
	git_folder: PathBuf
}

impl Git {
	fn find_first_path_with_git_folder(folder: Option<PathBuf>) -> Option<PathBuf> {
		if let Some(folder) = folder {
			let git_path = format!("{}/.git", folder.to_str().unwrap());
			let p = Path::new(&git_path).to_owned();
			if p.exists() {
				return Some(p)
			}

			if let Some(parent) = folder.parent() {
				return Git::find_first_path_with_git_folder(Some(parent.to_owned()))
			}
		}
		None
	}

	pub fn new() -> Self {
		let cwd = env::current_dir().unwrap();
		let git_folder = Git::find_first_path_with_git_folder(Some(cwd)).expect("unable to find .git folder");
		println!("{:?}", git_folder);
		Git { git_folder }
	}

	pub fn get_current_branch(&mut self) -> Result<String, Error> {
		let f = self.git_folder.join("HEAD");

		let head_file = File::open(f)?;
		let mut buf_reader = BufReader::new(head_file);
		let mut content = String::new();
		buf_reader.read_to_string(&mut content)?;
		Ok(content)
	}

	pub fn get_recent_branches(&mut self, limit: u32) -> Result<Vec<BranchHistory>, Error> {
		let mut ret: Vec<BranchHistory> = vec![];
		let f = self.git_folder.join("logs/HEAD");

		if !f.exists() {
			return Err(Error::MissingHeadLog)
		}

		let head_log_file = File::open(f)?;
		let mut buf_reader = BufReader::new(head_log_file);

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
						continue
					}

					let msg = &msg["checkout: moving from ".len()..msg.len()];

					let parts: Vec<&str> = msg.split(" to ").collect();
					if parts.len() != 2 {
						return Err(Error::MalformedCheckoutLog);
					}

					ret.push(BranchHistory{
						from: String::from(parts[0]),
						to: String::from(parts[1]),
					})
				},
				None => continue,
			}
		}

		Ok(ret)
	}
}
