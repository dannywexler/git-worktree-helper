use std::process::ExitCode;

use crate::{
    cli::{CLI, GWTCloneCommand, GWTCommand},
    git::{handle_clone_branch, handle_clone_repo},
    logging::StringResult,
    safe_fs::must_create_dir,
};
use clap::Parser;

pub mod cli;
pub mod git;
pub mod logging;
pub mod safe_fs;

fn run_cli() -> StringResult {
    let cli = CLI::parse();
    let code_dir = must_create_dir(cli.code_dir)?;
    match cli.command {
        GWTCommand::Clone { clone_target } => match clone_target {
            GWTCloneCommand::Repo { project, repo } => {
                handle_clone_repo(code_dir, &cli.host, &project, &repo)
            }
            GWTCloneCommand::Branch {
                project,
                repo,
                branch,
            } => handle_clone_branch(code_dir, cli.host, project, repo, branch),
        },
    }
}

fn main() -> ExitCode {
    match run_cli() {
        Ok(()) => ExitCode::SUCCESS,
        Err(original_msg) => {
            let msg = if original_msg.starts_with("Fatal") {
                original_msg
            } else {
                let mut new_msg = String::from("Fatal Error! ");
                new_msg.push_str(&original_msg);
                new_msg
            };
            eprintln!("{}", msg);
            ExitCode::FAILURE
        }
    }
}
