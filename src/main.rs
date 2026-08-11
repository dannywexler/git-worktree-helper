use crate::{
    cli::{CLI, GWTCloneCommand, GWTCommand},
    git::{clone_branch, clone_repo},
    safe_fs::must_create_dir,
};
use clap::Parser;

pub mod cli;
pub mod git;
pub mod logging;
pub mod safe_fs;

fn main() {
    let cli = CLI::parse();
    let code_dir = must_create_dir(cli.code_dir);
    match cli.command {
        GWTCommand::Clone { clone_target } => match clone_target {
            GWTCloneCommand::Repo { project, repo } => {
                clone_repo(&code_dir, &cli.host, &project, &repo);
            }
            // _ => todo!()
            GWTCloneCommand::Branch {
                project,
                repo,
                branch,
            } => clone_branch(&code_dir, &cli.host, project, repo, branch),
        },
    }
}
