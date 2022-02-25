use std::{env, path::PathBuf};
use std::path::Path;
use clap::{Parser, Subcommand};

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

	pub fn get_current_branch() -> String {

	}
}
