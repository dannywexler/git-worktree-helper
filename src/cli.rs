use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct CLI {
    /// The directory to store all code in
    #[arg(long, default_value_t = "~/code_dir".into())]
    pub code_dir: String,

    /// The git host
    #[arg(long, default_value_t = "github.com".into())]
    pub host: String,

    #[command(subcommand)]
    pub command: GWTCommand,
}

#[derive(Subcommand)]
pub enum GWTCommand {
    /// Clone a git repo
    Clone {
        #[command(subcommand)]
        clone_target: GWTCloneCommand,
    },
}

#[derive(Subcommand)]
pub enum GWTCloneCommand {
    Repo {
        /// The project to clone
        project: String,

        /// The repo of the project to clone
        repo: String,
    },
    Branch {
        /// The project containing the repo to clone the branch from
        #[arg(short, long)]
        project: Option<String>,

        /// The repo of the project to clone the branch from
        #[arg(short, long)]
        repo: Option<String>,

        /// The branch of the repo to clone
        #[arg(short, long)]
        branch: Option<String>,
    },
}
