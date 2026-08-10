use clap::{Parser, Subcommand};

use crate::must::must_create_dir;

pub mod git;
pub mod logging;
pub mod must;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CLI {
    /// The directory to store all code in
    #[arg(long, default_value_t = "~/code_dir".into())]
    code_dir: String,

    /// The git host
    #[arg(long, default_value_t = "github.com".into())]
    host: String,

    #[command(subcommand)]
    command: GWTCommand,
}

#[derive(Subcommand)]
enum GWTCommand {
    /// Clone a git repo
    Clone {
        /// The project to clone
        project: String,

        /// The repo of the project to clone
        repo: String,
    },
}

fn main() {
    let cli = CLI::parse();
    let code_dir = must_create_dir(cli.code_dir);
    match cli.command {
        GWTCommand::Clone { project, repo } => {
            git::clone_repo(code_dir, &cli.host, &project, &repo);
        }
    }
}
